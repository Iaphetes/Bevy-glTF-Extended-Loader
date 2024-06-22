# Bevy GLTF Extended Loader (BGEL)

## What does this do?
This tool applies some (at the moment one) [Khronos glTF Extensions](https://github.com/KhronosGroup/glTF/blob/main/extensions/README.md) to the relevant materials.

Here is a list of the currently supported extensions. Feel free to ask for more or even open a pull request, if you have some implementation:
- [x] KHR_materials_emissive_strength

## Usage
- Add the `GLTFExtender` plugin
- Add a meta file to your gltf, which includes includes the ```include_sources: true,``` flag (see material_test.glb.meta)
- For every added gltf do the following:
```rust
  let my_gltf = assets.load("material_test.glb");
  commands.spawn(SpawnAsset { handle: my_gltf });
```

## Examples
### Emissive Example
```
cargo run --example emissive_example
```
Shows an emissive example with massive bloom.
![Without bloom](./resources/example_emissive_without.png)
![With bloom](./resources/example_emissive_with.png)
## Changes
### 0.1.0 Initial release
#### Features
- Simple support for KHR_materials_emissive_strength
  - Multiplies the emissive strength of the material by this value 

### 0.1.1
#### Changes
- The library is now internally also called bgel
- Fixed usage section in readme
