# Bevy Multiplayer Template
This is a template for kicking off a multiplayer game in bevy using [bevy_replicon](https://github.com/projectharmonia/bevy_replicon.git)

## Using this template
Use [cargo-generate](https://github.com/cargo-generate/cargo-generate) to generate a project from this template.
It will ask you questions that will be used to populate the variables in this template.
```
cargo generate paul-hansen/bevy_multiplayer_template
```

You can then run it using `cargo run -- host` and in a separate terminal `cargo run -- join`.
Or if you want to run both with one command, you can use the included Makefile.toml with [cargo-make](https://github.com/sagiegurari/cargo-make):
```
cargo make run-two
```

## Goals / TODO

- [x] Basic client and server connecting
- [x] Protocol ID generation from crate name and version
- [x] Optional bevy_egui_inspector support
- [ ] Wasm Support
- [ ] CI in generated project
- [ ] CI for this project
- [ ] bevy_enhanced_input support
- [ ] leafwing_input_manger support
- [ ] Introductory documentation
- [ ] Steamworks integration

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

