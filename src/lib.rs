//! This crate provides bindings to the [VTK](https://vtk.org) project.
//!
//! It depends on system libraries which need to be preinstalled.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod vtk_named_colors;
mod vtk_object_base;
mod vtk_sphere;

pub use vtk_named_colors::*;
pub use vtk_object_base::*;
pub use vtk_sphere::*;
