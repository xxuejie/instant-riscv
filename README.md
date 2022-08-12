# instant-riscv

A RISC-V interpreter running in the browser. This is powered by [ckb-vm](https://github.com/nervosnetwork/ckb-vm) with [RISC-V V extension](https://github.com/riscv/riscv-v-spec) enabled.

As the name suggested, we hope this can be an instant playground for RISC-V.

A live deployment of this repo can be found at [here](https://vsetv.li/).

# Architecture

With the power of [xterm-pty](https://github.com/mame/xterm-pty), instant-riscv employs a particular architecture, where only the code absolutely required by the UI(mostly xterm.js related code) runs on the main DOM thread, the RISC-V interpreter is actually running in a separate Web Worker thread.

While this architecture is not uncommon in modern GUI apps, it is not so common in Web apps. Hopefully we can bring some inspirations to sophisticated modern Web apps.
