#include <vtkNew.h>
#include <vtkSphereSource.h>

using vtkSphereSourcePointer = vtkNew<vtkSphereSource>;

extern "C" void *sphere_source_new();
extern "C" void sphere_source_delete(void *sphere_source_ptr);
extern "C" void sphere_source_set_radius(void *sphere_source_ptr, double radius);
extern "C" double sphere_source_get_radius(void *sphere_source_ptr);
extern "C" void sphere_source_set_center(void *sphere_source_ptr, double center[3]);
extern "C" void sphere_source_get_center(void *sphere_source_ptr, double *center);
extern "C" void sphere_source_set_phi_resolution(void *sphere_source_ptr, int resolution);
extern "C" void sphere_source_set_theta_resolution(void *sphere_source_ptr, int resolution);
extern "C" const char *sphere_source_print_self(void *sphere_source_ptr, unsigned int indent);

vtkSphereSourcePointer* sphere_source_new2();
void sphere_source_delete2(const vtkSphereSourcePointer &ptr);
