# dependcytest
 
this repository just for reproduce the error  that  `module imports a non-existent function` message when uploading wasm file to node.

using  ink version: `tag = "v2.0.0"`, node of substrate version is:  `version 2.0.0-alpha.4-0b3020796-x86_64-linux-gnu`

Proceed as follows:

1、 add `schnorrkel` dependency in `cargo.toml`,for example:`schnorrkel = { version = "0.9.1",features = ["preaudit_deprecated", "u64_backend"], default-features = false}`;

2、using `cargo contract build`  command generate wasm file and `cargo contract generate-matedata` generate metadata.json

3、start substrate node: `RUST_LOG="runtime=debug,substrate=error" ./substrate --dev`

4、open [app](https://polkadot.js.org/apps) in browser,connect local node;

5、uploading wasm file,terminal will appearance information:

```
2020-03-22 22:13:36.028 tokio-blocking-driver DEBUG runtime  DispatchError
2020-03-22 22:13:36.028 tokio-blocking-driver DEBUG runtime  module imports a non-existent function
```



