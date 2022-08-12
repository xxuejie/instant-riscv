# instant-riscv

A RISC-V interpreter running in the browser. This is powered by [ckb-vm](https://github.com/nervosnetwork/ckb-vm) with [RISC-V V extension](https://github.com/riscv/riscv-v-spec) enabled.

As the name suggested, we hope this can be an instant playground for RISC-V.

A live deployment of this repo can be found at [here](https://vsetv.li/).

# Architecture

With the power of [xterm-pty](https://github.com/mame/xterm-pty), instant-riscv employs a particular architecture, where only the code absolutely required by the UI(mostly xterm.js related code) runs on the main DOM thread, the RISC-V interpreter is actually running in a separate Web Worker thread.

While this architecture is not uncommon in modern GUI apps, it is not so common in Web apps. Hopefully we can bring some inspirations to sophisticated modern Web apps.

# Usage

Make sure you have [wasi-sdk] installed, and the environment variable `WASI_SDK_PATH` properly set.

`cargo-wasi` is also needed:

```
$ cargo install cargo-wasi
```

Now you can start building the project:

```
$ git clone --recursive https://github.com/xxuejie/instant-riscv
$ cd instant-riscv
$ cargo wasi build --release
$ cd web
$ npm i
$ npm run build
$ npm run start
```

A local server will be booted at <http://127.0.0.1:3000> for you to try out the interpreter.

While the above steps setup an environment to try out the interpreter from a browser, the exact same code can also be used for a native CLI program:

```
$ # navigate to the root of the repository first
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/instant-riscv`
Welcome to instant-riscv, a RISC-V interpreter running in Web Worker!
Type :help for help.
For more details, please visit the repository at https://github.com/xxuejie/instant-riscv
>>
```
