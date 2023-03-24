# Guardian Skill Report

A program to inform players of their Destiny 2 statistics in the form of an
immersive and game-like report.

## Usage

Traditional `cargo run` for local usage (though you can use the `--release`
flag for slower compilation and better preformance):


```
cargo run --release
```

To serve it from a Unix machine (will not work on windows without WSL):


```
cargo install --lock trunk
trunk serve
```

## Developer Notes

(to self, really)
Remember that you can hit `Ctrl` + `Shift` + `V` to toggle markdown preview
