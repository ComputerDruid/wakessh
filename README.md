wakessh is a connection helper for ssh that can run a wakeonlan script if the ssh target is too slow to respond

To configure, edit your `.ssh/config` to include something like:

```
Host MYCOMPUTER
    ProxyCommand wakessh %h %p -- ssh -q RASPBERRYPI wakeonlan MAC_ADDRESS
    ProxyUseFdPass yes
```

The command after the -- can be whatever you want. My usecase is to ssh into my
raspberry pi and run the wakeonlan from there, but if you're already local to
the computer you might be able to wake it directly.
