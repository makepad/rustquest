#![cfg(target_os = "android")]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::mem;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub unsafe fn vrapi_DefaultInitParms(java: *const ovrJava) -> ovrInitParms {
    let mut parms: ovrInitParms = mem::zeroed();

    parms.Type = ovrStructureType__VRAPI_STRUCTURE_TYPE_INIT_PARMS;
    parms.ProductVersion = VRAPI_PRODUCT_VERSION as i32;
    parms.MajorVersion = VRAPI_MAJOR_VERSION as i32;
    parms.MinorVersion = VRAPI_MINOR_VERSION as i32;
    parms.PatchVersion = VRAPI_PATCH_VERSION as i32;
    parms.GraphicsAPI = ovrGraphicsAPI__VRAPI_GRAPHICS_API_OPENGL_ES_2;
    parms.Java = *java;

    parms
}

pub unsafe fn vrapi_DefaultModeParms(java: *const ovrJava) -> ovrModeParms {
    let mut parms: ovrModeParms = mem::zeroed();

    parms.Type = ovrStructureType__VRAPI_STRUCTURE_TYPE_MODE_PARMS;
    parms.Flags |= ovrModeFlags__VRAPI_MODE_FLAG_ALLOW_POWER_SAVE;
    parms.Flags |= ovrModeFlags__VRAPI_MODE_FLAG_RESET_WINDOW_FULLSCREEN;
    parms.Java = *java;

    parms
}
