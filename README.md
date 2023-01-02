[『競技プログラミングの鉄則』](https://book.mynavi.jp/ec/products/detail/id=131288)をRustでやっていく。

設定

```
> rustup install 1.42.0
> rustup default 1.42.0
```

rust-analyzer 用設定

```
> rustup component add rust-src
> cd $HOME/.rustup/toolchains/1.42.0-x86_64-unknown-linux-gnu/lib/rustlib/src/rust
> ln -sv src library
```
