//! This crate provides bindings to the [VTK](https://vtk.org) project.
//!
//! It depends on system libraries which need to be preinstalled.

mod macros;
mod vtk_named_colors;
mod vtk_object;
mod vtk_poly_data_mapper;
mod vtk_sphere;
mod vtk_sphere_source;

use macros::*;
pub use vtk_named_colors::*;
pub use vtk_object::*;
pub use vtk_poly_data_mapper::*;
pub use vtk_sphere::*;
pub use vtk_sphere_source::*;
