# Rust async

## 参考

- [Async in depth | Tokio - An asynchronous Rust runtime](https://tokio.rs/tokio/tutorial/async)
- [Async をさらに掘り下げる｜Tokio チュートリアル (日本語訳)](https://zenn.dev/magurotuna/books/tokio-tutorial-ja/viewer/async_in_depth)


## async/await

async関数は`Future`型を返す。


## std::future::Future

`Future` traitの定義は、以下のとおり。

```
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

- `Output`: `Future`完了時の出力の型
- `Pin`: async関数の中で借用を実現するための型
- `poll`: `Future`の所有者がポーリングするときに呼び出す。
  - `poll`が呼び出された時に値が決定できる状態であれば、値を計算し`Poll::Ready<Output>`を返す
  - `poll`が呼び出された時に値が決定できる状態でなければ、`Poll::Pending`を返す

## Executor

`Future::poll`を呼び出すオブジェクト。
`tokio::spawn`など。
