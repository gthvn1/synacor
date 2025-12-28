- The [Synacor Challenge](https://web.archive.org/web/20230206005149/https://challenge.synacor.com/) is a fun online coding challenge.
- The original website isn’t available anymore.
- I found out about it through [stewSquared's YouTube channel](https://www.youtube.com/@stewSquared).
- This is a great way to practice and learn Rust while solving the challenge.

```sh
❯ cargo run  --  roms/challenge.bin
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/synacor roms/challenge.bin`
30050 words loaded in memory from roms/challenge.bin
[b]reak, [c]ontinue, [p]rint, [q]uit, [r]un, [s]tep
debug> p
=> Mem[00000 (0x00000)]: 0x00015
   Mem[00001 (0x00001)]: 0x00015
   Mem[00002 (0x00002)]: 0x00013
   Mem[00003 (0x00003)]: 0x00057
   Mem[00004 (0x00004)]: 0x00013
   Mem[00005 (0x00005)]: 0x00065

debug> r
Welcome to the Synacor Challenge!
Please record your progress by putting codes like
this one into the challenge website: gmpJPiyErvJM

Executing self-test...

no jt/jf
debug> p
   Mem[01088 (0x00440)]: 0x00013
   Mem[01089 (0x00441)]: 0x00066
   Mem[01090 (0x00442)]: 0x00013
   Mem[01091 (0x00443)]: 0x0000a
   Mem[01092 (0x00444)]: 0x00000
=> Mem[01093 (0x00445)]: 0x00013
   Mem[01094 (0x00446)]: 0x0006e
   Mem[01095 (0x00447)]: 0x00013
   Mem[01096 (0x00448)]: 0x0006f
   Mem[01097 (0x00449)]: 0x00013
   Mem[01098 (0x0044A)]: 0x0006e

debug>
```
