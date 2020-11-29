https://nindanaoto.github.io
https://blog.vippool.net/entry/2020/06/08/155013

```sh
cargo test --featutes -- --nocapture nand
cargo test --features=fft -- --nocapture nand
cargo test --features=spqlios -- --nocapture nand
cargo bench
cargo bench --features=fft
cargo bench --features=spqlios
```

## TODO
- [x] fft
- [x] spqliosを使う
- [ ] reduce todo, fixme
- [ ] 鍵のサイズを図ってみる

  - https://qiita.com/nindanaoto/items/1023c1a490b818bd2ddd#tfhe%E3%81%A7spqlios%E3%82%92%E7%94%A8%E3%81%84%E3%81%A6%E3%81%84%E3%82%8B%E3%81%A8%E3%81%8D%E3%81%AE%E3%82%A2%E3%83%AB%E3%82%B4%E3%83%AA%E3%82%BA%E3%83%A0
