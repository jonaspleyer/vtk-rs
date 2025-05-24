extern "C" void *sphere_new();
extern "C" void sphere_delete(void *sphere_ptr);
extern "C" void sphere_set_radius(void *sphere_ptr, double radius);
extern "C" double sphere_get_radius(void *sphere_ptr);
extern "C" void sphere_set_center(void *sphere_ptr, double center[3]);
extern "C" void sphere_get_center(void *sphere_ptr, double *center);
extern "C" const char *sphere_print_self(void *sphere_ptr, unsigned int indent);
