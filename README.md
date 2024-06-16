# qlock-rs

qlock-rs is a text-based clock that displays the time by lighting up letters using ANSI escape sequences, and... the background text is a source code of itself, i.e. a [quine](<https://en.wikipedia.org/wiki/Quine_(computing)>).

It is inspired by the YouTube video by Tsoding: <https://www.youtube.com/watch?v=plFwBqBYpcY>, which is inspired by JavaScript version <https://x.com/aemkei/status/1795573193559994505> from Martin Kleppe.

# Usage

```
cargo xtask run
```

will "compile" the qlock_base.rs into qlock.rs, and run it. The "compiler" minifies the qlock_base.rs and embeds source code to make qlock.rs quine.

# Other usages

The project actually includes two programs, qlock_base.rs and quine_base.rs. quine_base.rs is a proof of concept to check the "compiler" works as intended. It is neither a clock nor used in the qlock.

```
cargo xtask compile
```

compiles qlock_base.rs and quine_base.rs into qlock.rs and quine.rs, respectively. The generated code are quine.

```
cargo xtask diff
```

will compare quine_base.rs.

# Under the hood

It is based on the same idea in the YouTube video by Tsoding, though I picked `~` as a "self-printing" operator instead of `?` to make use of debug output feature `{:?}` in Rust. It is the "compiler" that replaces `~` with the minified version of the source code while he hand-crafted this in his video.

# License

MIT License
