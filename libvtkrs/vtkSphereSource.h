extern "C" void *sphere_source_new();
extern "C" void sphere_source_delete(void *sphere_source_ptr);
extern "C" void sphere_source_set_radius(void *sphere_source_ptr, double radius);
extern "C" double sphere_source_get_radius(void *sphere_source_ptr);
extern "C" void sphere_source_set_center(void *sphere_source_ptr, double center[3]);
extern "C" double *sphere_source_get_center(void *sphere_source_ptr, double *center);
extern "C" void sphere_source_print_self(void *sphere_source_ptr, unsigned int indent);
