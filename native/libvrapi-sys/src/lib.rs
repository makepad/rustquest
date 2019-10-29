#![cfg(target_os = "android")]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::f32;
use std::f32::consts::PI;
use std::mem;
use std::ptr;

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

pub unsafe fn vrapi_DefaultLayerProjection2() -> ovrLayerProjection2 {
    let mut layer: ovrLayerProjection2 = mem::zeroed();

    let projectionMatrix = ovrMatrix4f_CreateProjectionFov(90.0, 90.0, 0.0, 0.0, 0.1, 0.0);
    let texCoordsFromTanAngles = ovrMatrix4f_TanAngleMatrixFromProjection(&projectionMatrix);

    layer.Header.Type = ovrLayerType2__VRAPI_LAYER_TYPE_PROJECTION2;
    layer.Header.Flags = 0;
    layer.Header.ColorScale.x = 1.0;
    layer.Header.ColorScale.y = 1.0;
    layer.Header.ColorScale.z = 1.0;
    layer.Header.ColorScale.w = 1.0;
    layer.Header.SrcBlend = ovrFrameLayerBlend__VRAPI_FRAME_LAYER_BLEND_ONE;
    layer.Header.DstBlend = ovrFrameLayerBlend__VRAPI_FRAME_LAYER_BLEND_ZERO;
    layer.Header.Reserved = ptr::null_mut();

    layer.HeadPose.Pose.Orientation.w = 1.0;

    for i in 0..ovrFrameLayerEye__VRAPI_FRAME_LAYER_EYE_MAX as usize {
        layer.Textures[i].TexCoordsFromTanAngles = texCoordsFromTanAngles;
        layer.Textures[i].TextureRect.x = 0.0;
        layer.Textures[i].TextureRect.y = 0.0;
        layer.Textures[i].TextureRect.width = 1.0;
        layer.Textures[i].TextureRect.height = 1.0;
    }

    layer
}

pub unsafe fn vrapi_DefaultModeParms(java: *const ovrJava) -> ovrModeParms {
    let mut parms: ovrModeParms = mem::zeroed();

    parms.Type = ovrStructureType__VRAPI_STRUCTURE_TYPE_MODE_PARMS;
    parms.Flags |= ovrModeFlags__VRAPI_MODE_FLAG_ALLOW_POWER_SAVE;
    parms.Flags |= ovrModeFlags__VRAPI_MODE_FLAG_RESET_WINDOW_FULLSCREEN;
    parms.Java = *java;

    parms
}

pub unsafe fn ovrMatrix4f_CreateProjection(
    minX: f32,
    maxX: f32,
    minY: f32,
    maxY: f32,
    nearZ: f32,
    farZ: f32,
) -> ovrMatrix4f {
    let width = maxX - minX;
    let height = maxY - minY;
    let offsetZ = nearZ;

    let mut out: ovrMatrix4f = mem::zeroed();
    if farZ <= nearZ {
        out.M[0][0] = 2.0 * nearZ / width;
        out.M[0][1] = 0.0;
        out.M[0][2] = (maxX + minX) / width;
        out.M[0][3] = 0.0;

        out.M[1][0] = 0.0;
        out.M[1][1] = 2.0 * nearZ / height;
        out.M[1][2] = (maxY + minY) / height;
        out.M[1][3] = 0.0;

        out.M[2][0] = 0.0;
        out.M[2][1] = 0.0;
        out.M[2][2] = -1.0;
        out.M[2][3] = -(nearZ + offsetZ);

        out.M[3][0] = 0.0;
        out.M[3][1] = 0.0;
        out.M[3][2] = -1.0;
        out.M[3][3] = 0.0;
    } else {
        out.M[0][0] = 2.0 * nearZ / width;
        out.M[0][1] = 0.0;
        out.M[0][2] = (maxX + minX) / width;
        out.M[0][3] = 0.0;

        out.M[1][0] = 0.0;
        out.M[1][1] = 2.0 * nearZ / height;
        out.M[1][2] = (maxY + minY) / height;
        out.M[1][3] = 0.0;

        out.M[2][0] = 0.0;
        out.M[2][1] = 0.0;
        out.M[2][2] = -(farZ + offsetZ) / (farZ - nearZ);
        out.M[2][3] = -(farZ * (nearZ + offsetZ)) / (farZ - nearZ);

        out.M[3][0] = 0.0;
        out.M[3][1] = 0.0;
        out.M[3][2] = -1.0;
        out.M[3][3] = 0.0;
    }
    out
}

pub unsafe fn ovrMatrix4f_CreateProjectionFov(
    fovDegreesX: f32,
    fovDegreesY: f32,
    offsetX: f32,
    offsetY: f32,
    nearZ: f32,
    farZ: f32,
) -> ovrMatrix4f {
    let halfWidth = nearZ * f32::tan(fovDegreesX * (PI / 180.0 * 0.5));
    let halfHeight = nearZ * f32::tan(fovDegreesY * (PI / 180.0 * 0.5));

    let minX = offsetX - halfWidth;
    let maxX = offsetX + halfWidth;

    let minY = offsetY - halfHeight;
    let maxY = offsetY + halfHeight;

    ovrMatrix4f_CreateProjection(minX, maxX, minY, maxY, nearZ, farZ)
}

pub unsafe fn ovrMatrix4f_CreateTranslation(x: f32, y: f32, z: f32) -> ovrMatrix4f {
    let mut out: ovrMatrix4f = mem::zeroed();
    out.M[0][0] = 1.0;
    out.M[0][1] = 0.0;
    out.M[0][2] = 0.0;
    out.M[0][3] = x;
    out.M[1][0] = 0.0;
    out.M[1][1] = 1.0;
    out.M[1][2] = 0.0;
    out.M[1][3] = y;
    out.M[2][0] = 0.0;
    out.M[2][1] = 0.0;
    out.M[2][2] = 1.0;
    out.M[2][3] = z;
    out.M[3][0] = 0.0;
    out.M[3][1] = 0.0;
    out.M[3][2] = 0.0;
    out.M[3][3] = 1.0;
    out
}

pub unsafe fn ovrMatrix4f_CreateFromQuaternion(q: *const ovrQuatf) -> ovrMatrix4f {
    let ww = (*q).w * (*q).w;
    let xx = (*q).x * (*q).x;
    let yy = (*q).y * (*q).y;
    let zz = (*q).z * (*q).z;

    let mut out = mem::zeroed::<ovrMatrix4f>();
    out.M[0][0] = ww + xx - yy - zz;
    out.M[0][1] = 2.0 * ((*q).x * (*q).y - (*q).w * (*q).z);
    out.M[0][2] = 2.0 * ((*q).x * (*q).z + (*q).w * (*q).y);
    out.M[0][3] = 0.0;

    out.M[1][0] = 2.0 * ((*q).x * (*q).y + (*q).w * (*q).z);
    out.M[1][1] = ww - xx + yy - zz;
    out.M[1][2] = 2.0 * ((*q).y * (*q).z - (*q).w * (*q).x);
    out.M[1][3] = 0.0;

    out.M[2][0] = 2.0 * ((*q).x * (*q).z - (*q).w * (*q).y);
    out.M[2][1] = 2.0 * ((*q).y * (*q).z + (*q).w * (*q).x);
    out.M[2][2] = ww - xx - yy + zz;
    out.M[2][3] = 0.0;

    out.M[3][0] = 0.0;
    out.M[3][1] = 0.0;
    out.M[3][2] = 0.0;
    out.M[3][3] = 1.0;
    out
}

pub unsafe fn ovrMatrix4f_Multiply(a: *const ovrMatrix4f, b: *const ovrMatrix4f) -> ovrMatrix4f {
    let mut out = mem::zeroed::<ovrMatrix4f>();
    out.M[0][0] = (*a).M[0][0] * (*b).M[0][0]
        + (*a).M[0][1] * (*b).M[1][0]
        + (*a).M[0][2] * (*b).M[2][0]
        + (*a).M[0][3] * (*b).M[3][0];
    out.M[1][0] = (*a).M[1][0] * (*b).M[0][0]
        + (*a).M[1][1] * (*b).M[1][0]
        + (*a).M[1][2] * (*b).M[2][0]
        + (*a).M[1][3] * (*b).M[3][0];
    out.M[2][0] = (*a).M[2][0] * (*b).M[0][0]
        + (*a).M[2][1] * (*b).M[1][0]
        + (*a).M[2][2] * (*b).M[2][0]
        + (*a).M[2][3] * (*b).M[3][0];
    out.M[3][0] = (*a).M[3][0] * (*b).M[0][0]
        + (*a).M[3][1] * (*b).M[1][0]
        + (*a).M[3][2] * (*b).M[2][0]
        + (*a).M[3][3] * (*b).M[3][0];

    out.M[0][1] = (*a).M[0][0] * (*b).M[0][1]
        + (*a).M[0][1] * (*b).M[1][1]
        + (*a).M[0][2] * (*b).M[2][1]
        + (*a).M[0][3] * (*b).M[3][1];
    out.M[1][1] = (*a).M[1][0] * (*b).M[0][1]
        + (*a).M[1][1] * (*b).M[1][1]
        + (*a).M[1][2] * (*b).M[2][1]
        + (*a).M[1][3] * (*b).M[3][1];
    out.M[2][1] = (*a).M[2][0] * (*b).M[0][1]
        + (*a).M[2][1] * (*b).M[1][1]
        + (*a).M[2][2] * (*b).M[2][1]
        + (*a).M[2][3] * (*b).M[3][1];
    out.M[3][1] = (*a).M[3][0] * (*b).M[0][1]
        + (*a).M[3][1] * (*b).M[1][1]
        + (*a).M[3][2] * (*b).M[2][1]
        + (*a).M[3][3] * (*b).M[3][1];

    out.M[0][2] = (*a).M[0][0] * (*b).M[0][2]
        + (*a).M[0][1] * (*b).M[1][2]
        + (*a).M[0][2] * (*b).M[2][2]
        + (*a).M[0][3] * (*b).M[3][2];
    out.M[1][2] = (*a).M[1][0] * (*b).M[0][2]
        + (*a).M[1][1] * (*b).M[1][2]
        + (*a).M[1][2] * (*b).M[2][2]
        + (*a).M[1][3] * (*b).M[3][2];
    out.M[2][2] = (*a).M[2][0] * (*b).M[0][2]
        + (*a).M[2][1] * (*b).M[1][2]
        + (*a).M[2][2] * (*b).M[2][2]
        + (*a).M[2][3] * (*b).M[3][2];
    out.M[3][2] = (*a).M[3][0] * (*b).M[0][2]
        + (*a).M[3][1] * (*b).M[1][2]
        + (*a).M[3][2] * (*b).M[2][2]
        + (*a).M[3][3] * (*b).M[3][2];

    out.M[0][3] = (*a).M[0][0] * (*b).M[0][3]
        + (*a).M[0][1] * (*b).M[1][3]
        + (*a).M[0][2] * (*b).M[2][3]
        + (*a).M[0][3] * (*b).M[3][3];
    out.M[1][3] = (*a).M[1][0] * (*b).M[0][3]
        + (*a).M[1][1] * (*b).M[1][3]
        + (*a).M[1][2] * (*b).M[2][3]
        + (*a).M[1][3] * (*b).M[3][3];
    out.M[2][3] = (*a).M[2][0] * (*b).M[0][3]
        + (*a).M[2][1] * (*b).M[1][3]
        + (*a).M[2][2] * (*b).M[2][3]
        + (*a).M[2][3] * (*b).M[3][3];
    out.M[3][3] = (*a).M[3][0] * (*b).M[0][3]
        + (*a).M[3][1] * (*b).M[1][3]
        + (*a).M[3][2] * (*b).M[2][3]
        + (*a).M[3][3] * (*b).M[3][3];
    out
}

pub unsafe fn ovrMatrix4f_TanAngleMatrixFromProjection(
    projection: *const ovrMatrix4f,
) -> ovrMatrix4f {
    let tanAngleMatrix = ovrMatrix4f {
        M: [
            [
                0.5 * (*projection).M[0][0],
                0.0,
                0.5 * (*projection).M[0][2] - 0.5,
                0.0,
            ],
            [
                0.0,
                0.5 * (*projection).M[1][1],
                0.5 * (*projection).M[1][2] - 0.5,
                0.0,
            ],
            [0.0, 0.0, -1.0, 0.0],
            [
                (*projection).M[2][2],
                (*projection).M[2][3],
                (*projection).M[3][2],
                1.0,
            ],
        ],
    };
    tanAngleMatrix
}

pub unsafe fn ovrMatrix4f_Transpose(a: *const ovrMatrix4f) -> ovrMatrix4f {
    let mut out: ovrMatrix4f = mem::zeroed();
    out.M[0][0] = (*a).M[0][0];
    out.M[0][1] = (*a).M[1][0];
    out.M[0][2] = (*a).M[2][0];
    out.M[0][3] = (*a).M[3][0];
    out.M[1][0] = (*a).M[0][1];
    out.M[1][1] = (*a).M[1][1];
    out.M[1][2] = (*a).M[2][1];
    out.M[1][3] = (*a).M[3][1];
    out.M[2][0] = (*a).M[0][2];
    out.M[2][1] = (*a).M[1][2];
    out.M[2][2] = (*a).M[2][2];
    out.M[2][3] = (*a).M[3][2];
    out.M[3][0] = (*a).M[0][3];
    out.M[3][1] = (*a).M[1][3];
    out.M[3][2] = (*a).M[2][3];
    out.M[3][3] = (*a).M[3][3];
    out
}
