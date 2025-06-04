#include <vtkNew.h>
#include <vtkSphereSource.h>

vtkSphereSource* vtk_sphere_source_new();
void vtk_sphere_source_delete(vtkSphereSource& sphere_source);
void vtk_sphere_source_set_radius(vtkSphereSource& sphere_source, double radius);
double vtk_sphere_source_get_radius(const vtkSphereSource& sphere_source);
void vtk_sphere_source_set_center(vtkSphereSource& sphere_source, std::array<double, 3>);
std::array<double, 3> vtk_sphere_source_get_center(const vtkSphereSource& sphere_source);
void vtk_sphere_source_set_phi_resolution(vtkSphereSource& sphere_source, int64_t resolution);
void vtk_sphere_source_set_theta_resolution(vtkSphereSource& sphere_source, int64_t resolution);
int64_t vtk_sphere_source_get_phi_resolution(const vtkSphereSource& sphere_source);
int64_t vtk_sphere_source_get_theta_resolution(const vtkSphereSource& sphere_source);
