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
// Taken from openvr.h
S_API bool VR_IsInterfaceVersionValid( const char *pchInterfaceVersion );

// Taken from openvr.h; use to work around broken ABI for this event in the C header
struct VREvent_Keyboard_t_real
{
	char cNewInput[8];	// Up to 11 bytes of new input
	uint64_t uUserValue;	// Possible flags about the new input
};

// Taken from openvr.h; use to work around broken ABI for this type in the C header
struct VRTextureWithPose_t_real
{
	void* handle; // See ETextureType definition above
	ETextureType eType;
	EColorSpace eColorSpace;
	HmdMatrix34_t mDeviceToAbsoluteTracking; // Actual pose used to render scene textures.
};
