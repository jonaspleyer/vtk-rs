#include <cstring>
#include <iostream>
#include <ostream>
#include <sstream>
#include <vtkSphere.h>
#include "sphere.h"

extern "C" void *sphere_new() { return vtkSphere::New(); }
extern "C" void sphere_delete(void *sphere_ptr) {
    vtkSphere *object;
    static_cast<vtkSphere *>(sphere_ptr)->Delete();
}

extern "C" void sphere_set_radius(void *sphere_ptr, double radius) {
    vtkSphere *object;
    static_cast<vtkSphere *>(sphere_ptr)->SetRadius(radius);
}

extern "C" double sphere_get_radius(void *sphere_ptr) {
    vtkSphere *object;
    return static_cast<vtkSphere *>(sphere_ptr)->GetRadius();
}

extern "C" void sphere_set_center(void *sphere_ptr, double center[3]) {
    vtkSphere *object;
    static_cast<vtkSphere *>(sphere_ptr)->SetCenter(center);
}
extern "C" void sphere_get_center(void *sphere_ptr, double *center) {
    vtkSphere *object;
    double *p = static_cast<vtkSphere *>(sphere_ptr)->GetCenter();
    memcpy(center, p, 3 * sizeof(double));
}

extern "C" const char *sphere_print_self(void *sphere_ptr, unsigned int indent) {
    vtkSphere *object;
    // This is a cast from the unsigned to signed int.
    // This should be fine
    vtkIndent ind = vtkIndent(indent);
    std::ostringstream oss;
    static_cast<vtkSphere *>(sphere_ptr)->PrintSelf(oss, ind);
    std::string out = oss.str();
    char *result    = new char[out.length() + 1];
    strcpy(result, out.c_str());
    return result;
}
