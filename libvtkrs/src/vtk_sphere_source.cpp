#include <cstring>
#include <iostream>
#include <ostream>
#include <sstream>

#include "cxx.h"
#include "vtk_sphere_source.h"
#include "vtk_sphere_source.rs.h"

#include <vtkNew.h>
#include <vtkSphereSource.h>

extern "C" void *sphere_source_new() { return vtkSphereSource::New(); }

extern "C" void sphere_source_delete(void *sphere_source_ptr) {
    static_cast<vtkSphereSource *>(sphere_source_ptr)->Delete();
}

extern "C" void sphere_source_set_radius(void *sphere_source_ptr, double radius) {
    static_cast<vtkSphereSource *>(sphere_source_ptr)->SetRadius(radius);
}

extern "C" double sphere_source_get_radius(void *sphere_source_ptr) {
    return static_cast<vtkSphereSource *>(sphere_source_ptr)->GetRadius();
}

extern "C" void sphere_source_set_center(void *sphere_source_ptr, double center[3]) {
    static_cast<vtkSphereSource *>(sphere_source_ptr)->SetCenter(center);
}
extern "C" void sphere_source_get_center(void *sphere_source_ptr, double *center) {
    double *p = static_cast<vtkSphereSource *>(sphere_source_ptr)->GetCenter();
    memcpy(center, p, 3 * sizeof(double));
}
extern "C" void sphere_source_set_phi_resolution(void *sphere_source_ptr, int resolution) {
    static_cast<vtkSphereSource *>(sphere_source_ptr)->SetPhiResolution(resolution);
}

extern "C" void sphere_source_set_theta_resolution(void *sphere_source_ptr, int resolution) {
    static_cast<vtkSphereSource *>(sphere_source_ptr)->SetThetaResolution(resolution);
}

extern "C" const char *sphere_source_print_self(void *sphere_source_ptr, unsigned int indent) {
    // This is a cast from the unsigned to signed int.
    // This should be fine
    vtkIndent ind = vtkIndent(indent);
    std::ostringstream oss;
    static_cast<vtkSphereSource *>(sphere_source_ptr)->PrintSelf(oss, ind);
    std::string out = oss.str();
    char *result    = new char[out.length() + 1];
    strcpy(result, out.c_str());
    return result;
}

// extern "C" vtkAlgorithmOutput *sphere_source_get_output_port(void *sphere_source_ptr) {
//     return static_cast<vtkSphereSource *>(sphere_source_ptr)->GetOutputPort();
// }

vtkSphereSourcePointer* sphere_source_new2() {
    vtkSphereSourcePointer *ptr = new vtkSphereSourcePointer();
    return ptr;
}

void sphere_source_delete2(vtkSphereSourcePointer &ptr) {
    ptr->Delete();
}
