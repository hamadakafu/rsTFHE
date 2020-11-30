## rsTFHE
The library implements Fast Fully Homomorphic Encryption Library over the Torus in Rust.

*version* 1.1.0 implements fft and using spqlios
*version* 1.0.0 very slow homnand

[benchmarks](https://hamadakafu.github.io/rsTFHE/report/index.html)

## How to Run
```
cargo run --example homnand --features=spqlios --release
```

```sh
cargo test -- --nocapture nand
cargo test --features=fft -- --nocapture nand
cargo test --features=spqlios -- --nocapture nand
cargo bench
cargo bench --features=fft # (x3)
cargo bench --features=spqlios # (x10)
```

## TODO
- [x] implements fft
- [x] implements spqlios
- [ ] reduce todo, fixme
- [ ] mesure key size
  - bench-rs?

## References
https://nindanaoto.github.io

https://blog.vippool.net/entry/2020/06/08/155013

I. Chillotti, N. Gama, M. Georgieva, and M. Izabachène. TFHE: Fast Fully Homomorphic Encryptionover the Torus. In Journal of Cryptology, volume 33, pages 34–91 (2020). [<span>PDF</span>](https://eprint.iacr.org/2018/421.pdf)

