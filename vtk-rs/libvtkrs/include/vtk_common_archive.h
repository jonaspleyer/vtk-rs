// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkBufferedArchiver.h>
#include<vtkPartitionedArchiver.h>

// Declare exported functions
extern "C" vtkNew < vtkBufferedArchiver > vtkBufferedArchiver_new () ;
extern "C" void vtkBufferedArchiver_destructor (vtkNew < vtkBufferedArchiver > sself) ;
extern "C" void * vtkBufferedArchiver_get_ptr (vtkNew < vtkBufferedArchiver > sself) ;
extern "C" vtkNew < vtkPartitionedArchiver > vtkPartitionedArchiver_new () ;
extern "C" void vtkPartitionedArchiver_destructor (vtkNew < vtkPartitionedArchiver > sself) ;
extern "C" void * vtkPartitionedArchiver_get_ptr (vtkNew < vtkPartitionedArchiver > sself) ;
