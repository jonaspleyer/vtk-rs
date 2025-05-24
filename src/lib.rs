//! This crate provides bindings to the [VTK](https://vtk.org) project.
//!
//! It depends on system libraries which need to be preinstalled.

mod poly_data_mapper;
mod vtk_named_colors;
mod vtk_sphere;
mod vtk_sphere_source;

pub use poly_data_mapper::*;
pub use vtk_named_colors::*;
pub use vtk_sphere::*;
pub use vtk_sphere_source::*;
