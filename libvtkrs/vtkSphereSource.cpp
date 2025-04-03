#include <cstring>
#include <iostream>
#include <ostream>
#include <sstream>
#include <vtkSphereSource.h>

extern "C" void *sphere_source_new() { return vtkSphereSource::New(); }
extern "C" void sphere_source_delete(void *sphere_source_ptr) {
    vtkSphereSource *object;
    static_cast<vtkSphereSource *>(sphere_source_ptr)->Delete();
}

extern "C" void sphere_source_set_radius(void *sphere_source_ptr, double radius) {
    vtkSphereSource *object;
    static_cast<vtkSphereSource *>(sphere_source_ptr)->SetRadius(radius);
}

extern "C" double sphere_source_get_radius(void *sphere_source_ptr) {
    vtkSphereSource *object;
    return static_cast<vtkSphereSource *>(sphere_source_ptr)->GetRadius();
}

extern "C" void sphere_source_set_center(void *sphere_source_ptr, double center[3]) {
    vtkSphereSource *object;
    static_cast<vtkSphereSource *>(sphere_source_ptr)->SetCenter(center);
}
extern "C" void sphere_source_get_center(void *sphere_source_ptr, double *center) {
    vtkSphereSource *object;
    double *p = static_cast<vtkSphereSource *>(sphere_source_ptr)->GetCenter();
    memcpy(center, p, 3 * sizeof(double));
}

extern "C" const char *sphere_source_print_self(void *sphere_source_ptr, unsigned int indent) {
    vtkSphereSource *object;
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
