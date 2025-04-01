extern "C" void object_delete(void *object);
extern "C" void object_fast_delete(void *object);
extern "C" void *object_new();
extern "C" const char *object_get_class_name(void *object_ptr);
extern "C" const char *object_get_object_description(void *object_ptr);
