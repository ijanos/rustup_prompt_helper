# rustup prompt helper

A small command line utility that prints the name of the selected rust toolchain
for the current working directory. It helps creating informative command line
prompts or scripts  where the alternative is piping the output of `rustup`
command through other tools. While that approach is fine it is slower and if you
have a busy prompt it can create a noticeable delay every time you press enter.

The output of `rustup_prompt_helper` should be identical to the following
command, as of rustup version 0.6.3

```
rustup show | tail -n 3 | head -n 1 |  cut -d '-' -f 1
```

## Example usage

You can install `rustup_prompt_helper` with the cargo. Make sure that
cargo's binary directory is part of your PATH environment variable.

```
cargo install rustup_prompt_helper
```

### bash

Here is simple bash prompt example. Place it in your `~/.bashrc` and it will
show the currently selected toolchain in the prompt if the directory contains
a `Cargo.toml` file.

```
function rust_toolchain {
   if [ -f Cargo.toml ]; then
      echo " [rust:$(rustup_prompt_helper)]"
   fi
}

PS1='[\u@\h \W]$(rust_toolchain) \$ '
```

### fish

Fish shell uses the `fish_prompt` function to create a prompt. It can be
customized by creating a `.config/fish/functions/fish_prompt.fish` file.

For a complete example see
[my fish prompt config](https://github.com/ijanos/dotfiles/blob/master/fish/.config/fish/functions/fish_prompt.fish).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
