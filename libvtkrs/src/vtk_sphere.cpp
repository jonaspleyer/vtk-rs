#include "cxx.h"
#include "vtk_sphere.h"
#include "vtk_sphere.rs.h"

vtkSphere* sphere_new() {
    return vtkSphere::New();
}

void sphere_delete(vtkSphere* sphere) {
    sphere->Delete();
}

void sphere_delete_pin(vtkSphere &sphere) {
    sphere.Delete();
}

double sphere_get_radius(vtkSphere &sphere) {
    return sphere.GetRadius();
}

void sphere_set_radius(vtkSphere &sphere, double radius) {
    sphere.SetRadius(radius);
}

std::array<double, 3> sphere_get_center(vtkSphere &sphere) {
    double* center = sphere.GetCenter();
    return std::array<double, 3>({center[0], center[1], center[2]});
}

void sphere_set_center(vtkSphere &sphere, std::array<double, 3> center) {
    sphere.SetCenter(center[0], center[1], center[2]);
}
