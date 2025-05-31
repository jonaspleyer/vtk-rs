#include <cstring>
#include <sstream>

#include "cxx.h"
#include "vtk_sphere_source.h"
#include "vtk_sphere_source.rs.h"

#include <vtkNew.h>
#include <vtkSphereSource.h>

vtkSphereSource* vtk_sphere_source_new() {
    return vtkSphereSource::New();
}

void vtk_sphere_source_delete(vtkSphereSource& sphere_source) {
    sphere_source.Delete();
}

void vtk_sphere_source_set_radius(vtkSphereSource& sphere_source, double radius) {
    sphere_source.SetRadius(radius);
}

double vtk_sphere_source_get_radius(const vtkSphereSource& sphere_source) {
    return const_cast<vtkSphereSource&>(sphere_source).GetRadius();
}

void vtk_sphere_source_set_center(vtkSphereSource& sphere_source, std::array<double, 3> center) {
    sphere_source.SetCenter(center.begin());
}

std::array<double, 3> vtk_sphere_source_get_center(const vtkSphereSource& sphere_source) {
    std::array<double, 3> center;
    double* ctr = const_cast<vtkSphereSource&>(sphere_source).GetCenter();
    std::copy(ctr, ctr + 3, center.begin());
    return center;
}

void vtk_sphere_source_set_phi_resolution(vtkSphereSource& sphere_source, int64_t phi_resolution) {
    sphere_source.SetPhiResolution(phi_resolution);
}

int64_t vtk_sphere_source_get_phi_resolution(const vtkSphereSource& sphere_source) {
    return const_cast<vtkSphereSource&>(sphere_source).GetPhiResolution();
}

void vtk_sphere_source_set_theta_resolution(
    vtkSphereSource& sphere_source, int64_t theta_resolution
) {
    sphere_source.SetThetaResolution(theta_resolution);
}

int64_t vtk_sphere_source_get_theta_resolution(const vtkSphereSource& sphere_source) {
    return const_cast<vtkSphereSource&>(sphere_source).GetThetaResolution();
}
