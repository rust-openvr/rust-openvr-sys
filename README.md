# openvr-sys

Contains function definitions for the [OpenVR](https://github.com/ValveSoftware/openvr) library. Use the `openvr` crate, unless you know what you are doing.

## Instructions for updating OpenVR

1. `git submodule update --init --recursive` (initial checkout only)
1. `git submodule foreach git pull origin master` to update the submodule 
1. `cargo build --features "buildtime_bindgen"` to update the bindings
1. Apply the workaround for broken OpenVR ABIs, if required

## Workaround for broken OpenVR ABIs (Linux/macOS only)

Search for packed structs in `headers/openvr.h`, i.e., `#pragma pack( push, 4 )`. Currently, that is:

```
VRControllerState_t
RenderModel_TextureMap_t
RenderModel_t
VREvent_t
```

Depending on what bindgen did parse, you have to replace a bunch of `#[repr(C)]` and `#[repr(C, packed(4))]` precending those structs in `bindings.rs` with:

```rust
#[repr(C)]
#[cfg_attr(unix, repr(packed(4)))]
```
