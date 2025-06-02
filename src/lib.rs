//! This crate provides bindings to the [VTK](https://vtk.org) project.
//!
//! It depends on system libraries which need to be preinstalled.

#![cfg_attr(docsrs, feature(doc_cfg))]

// Internal Tools
mod macros;

use macros::*;

// Exposed API
mod vtk_abstract_mapper;
mod vtk_abstract_mapper_3d;
mod vtk_algorithm;
mod vtk_algorithm_output;
mod vtk_data_object;
mod vtk_information;
mod vtk_information_vector;
mod vtk_mapper;
mod vtk_named_colors;
mod vtk_object;
mod vtk_object_base;
mod vtk_poly_data;
mod vtk_poly_data_algorithm;
mod vtk_poly_data_mapper;
mod vtk_sphere;
mod vtk_sphere_source;

pub use vtk_abstract_mapper::*;
pub use vtk_abstract_mapper_3d::*;
pub use vtk_algorithm::*;
pub use vtk_algorithm_output::*;
pub use vtk_data_object::*;
pub use vtk_information::*;
pub use vtk_information_vector::*;
pub use vtk_mapper::*;
pub use vtk_named_colors::*;
pub use vtk_object::*;
pub use vtk_object_base::*;
pub use vtk_poly_data::*;
pub use vtk_poly_data_algorithm::*;
pub use vtk_poly_data_mapper::*;
pub use vtk_sphere::*;
pub use vtk_sphere_source::*;
