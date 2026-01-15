// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkClientSocket.h>
#include<vtkDirectory.h>
#include<vtkExecutableRunner.h>
#include<vtkServerSocket.h>
#include<vtkSocket.h>
#include<vtkSocketCollection.h>
#include<vtkTimerLog.h>
#include<vtkTimerLog.h>
#include<vtkTimerLog.h>

// Declare exported functions
extern "C" vtkNew < vtkClientSocket > vtkClientSocket_new () ;
extern "C" void vtkClientSocket_destructor (vtkNew < vtkClientSocket > sself) ;
extern "C" void * vtkClientSocket_get_ptr (vtkNew < vtkClientSocket > sself) ;
extern "C" vtkNew < vtkDirectory > vtkDirectory_new () ;
extern "C" void vtkDirectory_destructor (vtkNew < vtkDirectory > sself) ;
extern "C" void * vtkDirectory_get_ptr (vtkNew < vtkDirectory > sself) ;
extern "C" vtkNew < vtkExecutableRunner > vtkExecutableRunner_new () ;
extern "C" void vtkExecutableRunner_destructor (vtkNew < vtkExecutableRunner > sself) ;
extern "C" void * vtkExecutableRunner_get_ptr (vtkNew < vtkExecutableRunner > sself) ;
extern "C" vtkNew < vtkServerSocket > vtkServerSocket_new () ;
extern "C" void vtkServerSocket_destructor (vtkNew < vtkServerSocket > sself) ;
extern "C" void * vtkServerSocket_get_ptr (vtkNew < vtkServerSocket > sself) ;
extern "C" vtkNew < vtkSocketCollection > vtkSocketCollection_new () ;
extern "C" void vtkSocketCollection_destructor (vtkNew < vtkSocketCollection > sself) ;
extern "C" void * vtkSocketCollection_get_ptr (vtkNew < vtkSocketCollection > sself) ;
extern "C" vtkNew < vtkTimerLog > vtkTimerLog_new () ;
extern "C" void vtkTimerLog_destructor (vtkNew < vtkTimerLog > sself) ;
extern "C" void * vtkTimerLog_get_ptr (vtkNew < vtkTimerLog > sself) ;
