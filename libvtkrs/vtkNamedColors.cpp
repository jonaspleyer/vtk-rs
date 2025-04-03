#include <stdio.h>
#include <vtkNamedColors.h>

extern "C" void *named_colors_new() {
    vtkNamedColors *colors = vtkNamedColors::New();
    return static_cast<void *>(colors);
}

extern "C" void named_colors_delete(void *object_ptr) {
    vtkNamedColors *object;
    static_cast<vtkNamedColors *>(object_ptr)->Delete();
}
