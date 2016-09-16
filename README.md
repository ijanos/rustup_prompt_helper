# rustup prompt helper

A small command line utility that prints the name of the selected rust
toolchain for the current working directory. Intended to help
create informative command line prompts or scripts  where the
alternative is piping the output of `rustup` command through other tools.

The output of `rustup_prompt_helper` should be identical to the following command, as of rustup version 0.6.3

```
rustup show | tail -n 3 | head -n 1 |  cut -d '-' -f 1
```


## Example usage

For a fish shell example usage please see
[my fish prompt config](https://github.com/ijanos/dotfiles/blob/master/fish/.config/fish/functions/fish_prompt.fish).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
