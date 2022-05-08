# エラー処理

## 参考

- プログラミングRust 第二版 7章
- [巻き戻し](https://doc.rust-jp.rs/rust-nomicon-ja/unwinding.html)

## エラー種別

- 通常のエラー：プログラム外部の要因（ネットワーク切断・権限など）による異常(c++でのstd::runtime_error)
  - Resultを使って問題を表現する
  - コンパイル時にエラーをハンドルしているかチェックしてくれる
  - 単純に伝搬したいときは?を使う
    - 複数種類のエラー型を返しうるときは、独自でエラー型を定義するか、`GenericError`型を返すようにするか
  - 起こるはずがないときだけ`unwrap`を使う
- パニック：プログラムのバグ(範囲外アクセスやゼロ除算など)による異常(c++でのstd::logic_error)
  - 処理１．スレッドの巻き戻し(unwind)
    - 概念的には例外に似ている
    - std::panic::catch_unwind()でキャッチ可能
    - スレッド単位で実行される
  - 処理２．アボート
    - プロセス全体を終了する
    - 下記の２つのケースでは、スタックの巻き戻しでは
    - パニックを巻き戻している最中にパニックが発生した場合
    - `-C panic=abort`を指定してコンパイルされた場合


## エラーのダンプ

`Error`を`println`で表示するだけでなく、`source`まで表示することで、
エラーの原因となったエラーの情報まで再帰的に取得できる可能性がある（`Error`型の実装次第)

```rust
use std::error::Error;
use std::io::{Write, stderr};

fn print_error(mut err: &dyn Error) {
  let _ = writeln!(stderr(), "error: {}", err);
  while let Some(source) = err.source() {
    let _ = writeln!(stderr(), "caused by: {}", source);
    err = source;
  }
}
```

## カスタムエラー型の宣言

[`thiserror`](https://docs.rs/thiserror/1.0.31/thiserror/) crateを使うのが簡単。
