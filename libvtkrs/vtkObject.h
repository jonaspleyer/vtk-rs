extern "C" void *object_new();
extern "C" void object_delete(void *object_ptr);
extern "C" void object_debug_on(void *object_ptr);
extern "C" void object_debug_off(void *object_ptr);
extern "C" bool object_get_debug(void *object_ptr);
extern "C" void object_set_debug(void *object_ptr, bool flag);
