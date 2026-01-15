// Include header file
#include<vtk_common_system.h>

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

// Implement declared functions
extern "C" vtkNew < vtkClientSocket > vtkClientSocket_new () {return vtkNew < vtkClientSocket > () ;}
extern "C" void vtkClientSocket_destructor (vtkNew < vtkClientSocket > sself) {sself . Reset () ; return ;}
extern "C" void * vtkClientSocket_get_ptr (vtkNew < vtkClientSocket > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDirectory > vtkDirectory_new () {return vtkNew < vtkDirectory > () ;}
extern "C" void vtkDirectory_destructor (vtkNew < vtkDirectory > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDirectory_get_ptr (vtkNew < vtkDirectory > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkExecutableRunner > vtkExecutableRunner_new () {return vtkNew < vtkExecutableRunner > () ;}
extern "C" void vtkExecutableRunner_destructor (vtkNew < vtkExecutableRunner > sself) {sself . Reset () ; return ;}
extern "C" void * vtkExecutableRunner_get_ptr (vtkNew < vtkExecutableRunner > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkServerSocket > vtkServerSocket_new () {return vtkNew < vtkServerSocket > () ;}
extern "C" void vtkServerSocket_destructor (vtkNew < vtkServerSocket > sself) {sself . Reset () ; return ;}
extern "C" void * vtkServerSocket_get_ptr (vtkNew < vtkServerSocket > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSocketCollection > vtkSocketCollection_new () {return vtkNew < vtkSocketCollection > () ;}
extern "C" void vtkSocketCollection_destructor (vtkNew < vtkSocketCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSocketCollection_get_ptr (vtkNew < vtkSocketCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTimerLog > vtkTimerLog_new () {return vtkNew < vtkTimerLog > () ;}
extern "C" void vtkTimerLog_destructor (vtkNew < vtkTimerLog > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTimerLog_get_ptr (vtkNew < vtkTimerLog > sself) {return sself . GetPointer () ;}
