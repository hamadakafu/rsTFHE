https://nindanaoto.github.io
https://blog.vippool.net/entry/2020/06/08/155013

```sh
cargo test --features=fft -- --nocapture nand
cargo bench --features=fft
```

## TODO
- [x] fft
- [ ] fftの虚数部分を使っていないので上N/2の分だけ虚数部分に押し込めれば節約できる
- [ ] parameterの管理
- [ ] reduce todo, fixme
- [ ] 鍵のサイズを図ってみる
- [ ] spqliosは半分を虚数部分に押し込めているのでそれを考慮して多項式積を書く
  - https://qiita.com/nindanaoto/items/1023c1a490b818bd2ddd#tfhe%E3%81%A7spqlios%E3%82%92%E7%94%A8%E3%81%84%E3%81%A6%E3%81%84%E3%82%8B%E3%81%A8%E3%81%8D%E3%81%AE%E3%82%A2%E3%83%AB%E3%82%B4%E3%83%AA%E3%82%BA%E3%83%A0
  - example で試す実行する
