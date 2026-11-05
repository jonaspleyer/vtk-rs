// Include header file
#include <vtk_common_archive.h>

// Default include in all modules
#include <vtkNew.h>
#include <vtkObjectBase.h>

// Include objects of this module
#include <vtkBufferedArchiver.h>
#include <vtkPartitionedArchiver.h>

// Implement declared functions
extern "C" vtkNew<vtkBufferedArchiver> vtkBufferedArchiver_new() {
    return vtkNew<vtkBufferedArchiver>();
}
extern "C" void vtkBufferedArchiver_destructor(vtkNew<vtkBufferedArchiver> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBufferedArchiver_get_ptr(vtkNew<vtkBufferedArchiver> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_buffered_archiver_set_allocated_size(
    vtkNew<vtkBufferedArchiver> self, size_t p0
) {
    return self->SetAllocatedSize(p0);
}
extern "C" size_t vtk_buffered_archiver_get_allocated_size(vtkNew<vtkBufferedArchiver> self) {
    return self->GetAllocatedSize();
}
extern "C" size_t vtk_buffered_archiver_get_buffer_size(vtkNew<vtkBufferedArchiver> self) {
    return self->GetBufferSize();
}
extern "C" vtkNew<vtkPartitionedArchiver> vtkPartitionedArchiver_new() {
    return vtkNew<vtkPartitionedArchiver>();
}
extern "C" void vtkPartitionedArchiver_destructor(vtkNew<vtkPartitionedArchiver> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPartitionedArchiver_get_ptr(vtkNew<vtkPartitionedArchiver> sself) {
    return sself.GetPointer();
}
extern "C" size_t vtk_partitioned_archiver_get_number_of_buffers(
    vtkNew<vtkPartitionedArchiver> self
) {
    return self->GetNumberOfBuffers();
}
