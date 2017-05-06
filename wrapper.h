// Generate bindings by running: bindgen --no-unstable-rust wrapper.h -o lib.rs

#include "openvr/headers/openvr_capi.h"

// Expose symbols hidden in C API for some reason; see
// https://github.com/ValveSoftware/openvr/issues/89
// https://steamcommunity.com/app/358720/discussions/0/405692758722144628/
// https://github.com/ValveSoftware/openvr/commit/6f671fc80947dbccc4a9d27bd1b89d6038d94637
S_API intptr_t VR_InitInternal( EVRInitError *peError, EVRApplicationType eType );
S_API void VR_ShutdownInternal();
S_API bool VR_IsHmdPresent();
S_API intptr_t VR_GetGenericInterface( const char *pchInterfaceVersion, EVRInitError *peError );
S_API bool VR_IsRuntimeInstalled();
S_API const char * VR_GetVRInitErrorAsSymbol( EVRInitError error );
S_API const char * VR_GetVRInitErrorAsEnglishDescription( EVRInitError error );
