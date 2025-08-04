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

## License

Copyright 2026 Daniel Johnson et al.

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE2](LICENSE-APACHE2) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
