# OBSOLETE!

This project is no longer maintained. It has been superseded by
[`bevy_common_assets`](https://github.com/NiklasEi/bevy_common_assets).

# Custom RON Assets for Bevy

This crate allows you to easily register arbitrary custom data to be loaded by
[Bevy](https://github.com/bevyengine/bevy) as an Asset, from files using the
[RON](https://github.com/ron-rs/ron) format.

It minimizes the amount of boilerplate needed for such custom asset types.

You only need to derive the required traits on your custom type, and add a
`RonAssetPlugin` to your `App`!

Caveat: you need to come up with a unique file name extension for each new asset
type. Bevy also requires a unique UUID for `TypeUuid`.

```rust
#[derive(serde::Deserialize)]
#[derive(TypeUuid)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
struct GameItemDescriptionAsset {
    damage: f32,
    durability: f32,
    min_level: u8,
}

fn main() {
    App::new()
        // bevy
        .add_plugins(DefaultPlugins)
        // our asset
        .add_plugin(
            // load `*.item` files
            RonAssetPlugin::<GameItemDescriptionAsset>::new(&["item"])
        )
        .add_startup_system(setup)
        .run();
}

fn setup(server: Res<AssetServer>) {
    // load our item configs!
    let handles = server.load_folder("items");

    // TODO: store the handles somewhere
}
```

Now you can just create files like `assets/items/big_gun.item`:

```
(
  damage: 25.0,
  durability: 170.0,
  min_level: 4,
)
```

---

See [`examples/load_rons.rs`](./examples/load_rons.rs) for a more elaborate example!

```
$ cargo run --example load_rons
```

## Compatible Bevy versions

Compatibility of published `bevy_asset_ron` versions:

| `bevy` | `bevy_asset_ron` |
|--------|------------------|
| `0.7`  | `0.4`            |
| `0.6`  | `0.3`            |
| `0.5`  | `0.2`            |
| `0.4`  | `0.1`            |
