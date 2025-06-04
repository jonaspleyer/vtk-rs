
#include "cxx.h"
#include <vtkSphere.h>

vtkSphere* sphere_new();
void sphere_delete(vtkSphere& sphere);
double sphere_get_radius(const vtkSphere& sphere);
void sphere_set_radius(vtkSphere& sphere, double radius);
std::array<double, 3> sphere_get_center(const vtkSphere& sphere);
void sphere_set_center(vtkSphere& sphere, std::array<double, 3> center);
