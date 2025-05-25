//! This crate provides bindings to the [VTK](https://vtk.org) project.
//!
//! It depends on system libraries which need to be preinstalled.

// Internal Tools
mod macros;

use macros::*;

// Exposed API
mod vtk_algorithm;
mod vtk_data_object;
mod vtk_named_colors;
mod vtk_object;
mod vtk_poly_data_mapper;
mod vtk_sphere;
mod vtk_sphere_source;

pub use vtk_algorithm::*;
pub use vtk_data_object::*;
pub use vtk_named_colors::*;
pub use vtk_object::*;
pub use vtk_poly_data_mapper::*;
pub use vtk_sphere::*;
pub use vtk_sphere_source::*;
