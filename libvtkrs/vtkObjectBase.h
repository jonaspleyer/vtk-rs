extern "C" void object_base_delete(void *object);
extern "C" void object_base_fast_delete(void *object);
extern "C" void *object_base_new();
extern "C" const char *object_base_get_class_name(void *object_ptr);
extern "C" const char *object_base_get_object_description(void *object_ptr);
