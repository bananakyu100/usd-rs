//------------------------------------------------------------------------------
// bananakyu100 : bananakyu100@gmail.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
use cpp::*;

#[cfg(target_family = "unix")]
cpp! {{
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
    #pragma GCC diagnostic ignored "-Wmissing-field-initializers"
    #include "pxr/usd/usdGeom/xform.h"
    #pragma GCC diagnostic pop
}}

#[cfg(target_family = "windows")]
cpp! {{
    #include "pxr/usd/usdGeom/xform.h"
}}

/*
use crate::pxr::sdf;
use crate::pxr::tf;

use crate::pxr::usd::prim::Prim;

use super::common::LoadPolicy;
use super::prim_range::*;


cpp_class!(pub unsafe struct UsdGeomXform as pxr::UsdGeomXform);

impl UsdGeomXform {
    pub fn 


}
*/
