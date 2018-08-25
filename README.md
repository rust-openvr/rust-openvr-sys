To update, update the submodule then bindgen on wrapper.h

The following structs must be marked `#[cfg_attr(unix, repr(packed))]` to work around OpenVR's broken ABI on Linux/OSX:
- `VRControllerState_t`
- `RenderModel_TextureMap_t`
- `RenderModel_t`
- `VREvent_t`
