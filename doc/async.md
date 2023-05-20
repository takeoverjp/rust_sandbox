# Rust async

## 参考

- [Async in depth | Tokio - An asynchronous Rust runtime](https://tokio.rs/tokio/tutorial/async)
- [Async をさらに掘り下げる｜Tokio チュートリアル (日本語訳)](https://zenn.dev/magurotuna/books/tokio-tutorial-ja/viewer/async_in_depth)

## RustにおけるThreadと非同期タスクの比較

- 生成に時間がかからない
- メモリのオーバーヘッドが少ない
  - スレッドの場合は、スレッドごとに用意されるスタックが100KiBとか。スレッドの多くがただシステムコールを待つだけの場合、相対的にスタックによるメモリコストが高くなる。
  - 

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

## Wakers

`Future`は、`Context::wakwer()`で取得した`Waker`に対して`wake()`を呼び出すことで、Executorに通知を飛ばし、タスクの再ポーリングを必要なときのみ（次に進むことができるときのみ）要求する。


```
Rust の非同期処理は "lazy" であり、呼び出し側がポーリングを行う必要がある
"future" と、その "future" を呼び出すタスクを紐付けるために "waker" が "future" に渡される
リソースが処理を完了させる準備が まだ 整っていない場合は、Poll::Pending が返され、タスクの "waker" が記録される
リソースの準備が整ったら、タスクの "waker" に通知が行く
実行器は通知を受け取り、タスクの実行をスケジュールする
タスクが再びポーリングされる。今度はリソースは準備が整っているので、タスクは次に進むことができる
```

## Select

`tokio::select!`マクロを使うことで、複数のasync式に対してそれぞれ完了時に実行したい処理を記述できる。  
最初に完了したasync式以外のasync式はキャンセルされる。  
必ず一つのasync式飲みが実行されるようにするため、すべての分岐は同一のタスク上で実行される。

## tokio_stream::StreamExt

streamとは`Future`のイテレータのことを指す。  
`while let`と`StreamExt::next()`で実現する。

`StreamExt::next()`を呼ぶためには、そのstreamがpinされている必要がある。  

残念ながらRust言語はまだ`Stream`を実装するasync/await構文をサポートしていない。  
代替策として、[async-stream crate](https://docs.rs/async-stream/latest/async_stream/)の`stream!`マクロを使うことで、yieldを使って記述できる。
