use std::{
    io::IoSlice,
    mem::MaybeUninit,
    net::TcpStream,
    os::fd::AsFd,
    time::Duration,
};

use clap::Parser;
use futures_concurrency::future::RaceOk;
use rustix::{
    io::fcntl_dupfd_cloexec,
    net::{SendAncillaryBuffer, SendAncillaryMessage, SendFlags, sendmsg},
    stdio::dup2_stdout,
};
use tokio::process::Command;

async fn connect(args: &Args) -> TcpStream {
    let (host, port) = (args.host.as_str(), args.port);
    let direct_connect = happy_eyeballs::tokio::connect((host, port));
    let wakeonlan_attempt = async {
        // don't bother trying to wakeonlan in the first 2 seconds
        tokio::time::sleep(Duration::from_secs(2)).await;
        eprintln!("wakessh: trying to wake {host}");
        let program = &args.wake_command[0];
        let mut sshcommand = Command::new(program)
            .args(&args.wake_command[1..])
            .kill_on_drop(true)
            .spawn()
            .expect("running ssh for wakeonlan");
        let exitcode = sshcommand.wait().await.expect("waiting for wakeonlan ssh");
        if !exitcode.success() {
            panic!("failed to ssh to jump host");
        }
        // wait a bit for the computer to resume from suspend
        tokio::time::sleep(Duration::from_secs(10)).await;
        let connection = happy_eyeballs::tokio::connect((host, port)).await?;

        Ok(connection)
    };
    let connection = (direct_connect, wakeonlan_attempt)
        .race_ok()
        .await
        .expect("all connection attempts");
    let connection = connection
        .into_std()
        .expect("extracting TcpStream from tokio runtime");
    connection
        .set_nonblocking(false)
        .expect("setting TcpStream back to nonblocking mode");
    connection
}

fn redirect_stdout_to_stderr() {
    dup2_stdout(std::io::stderr()).unwrap();
}

#[derive(Parser, Debug)]
struct Args {
    host: String,
    port: u16,
    #[arg(required = true)]
    wake_command: Vec<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    let outfd =
        fcntl_dupfd_cloexec(std::io::stdout(), 3).expect("getting new fd for output socket");
    redirect_stdout_to_stderr();

    let connection = connect(&args).await;
    let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(1))];
    let to_send = [connection.as_fd()];
    let mut cmsg_buffer = SendAncillaryBuffer::new(&mut space);
    cmsg_buffer.push(SendAncillaryMessage::ScmRights(&to_send));
    sendmsg(
        outfd,
        &[IoSlice::new(b"\0")],
        &mut cmsg_buffer,
        SendFlags::empty(),
    )
    .unwrap();
}
