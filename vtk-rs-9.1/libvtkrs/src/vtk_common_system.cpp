// Include header file
#include <vtk_common_system.h>

// Default include in all modules
#include <vtkNew.h>
#include <vtkObjectBase.h>

// Include objects of this module
#include <vtkClientSocket.h>
#include <vtkDirectory.h>
#include <vtkExecutableRunner.h>
#include <vtkServerSocket.h>
#include <vtkSocket.h>
#include <vtkSocketCollection.h>
#include <vtkTimerLog.h>

// Implement declared functions
extern "C" vtkNew<vtkClientSocket> vtkClientSocket_new() {
    return vtkNew<vtkClientSocket>();
}
extern "C" void vtkClientSocket_destructor(vtkNew<vtkClientSocket> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkClientSocket_get_ptr(vtkNew<vtkClientSocket> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_client_socket_get_connecting_side(vtkNew<vtkClientSocket> self) {
    return self->GetConnectingSide();
}
extern "C" vtkNew<vtkDirectory> vtkDirectory_new() {
    return vtkNew<vtkDirectory>();
}
extern "C" void vtkDirectory_destructor(vtkNew<vtkDirectory> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDirectory_get_ptr(vtkNew<vtkDirectory> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_directory_get_number_of_files(vtkNew<vtkDirectory> self) {
    return self->GetNumberOfFiles();
}
extern "C" vtkNew<vtkExecutableRunner> vtkExecutableRunner_new() {
    return vtkNew<vtkExecutableRunner>();
}
extern "C" void vtkExecutableRunner_destructor(vtkNew<vtkExecutableRunner> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkExecutableRunner_get_ptr(vtkNew<vtkExecutableRunner> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_executable_runner_execute(vtkNew<vtkExecutableRunner> self) {
    return self->Execute();
}
extern "C" void vtk_executable_runner_set_timeout(vtkNew<vtkExecutableRunner> self, double _arg) {
    return self->SetTimeout(_arg);
}
extern "C" double vtk_executable_runner_get_timeout(vtkNew<vtkExecutableRunner> self) {
    return self->GetTimeout();
}
extern "C" void vtk_executable_runner_set_right_trim_result(
    vtkNew<vtkExecutableRunner> self, bool _arg
) {
    return self->SetRightTrimResult(_arg);
}
extern "C" bool vtk_executable_runner_get_right_trim_result(vtkNew<vtkExecutableRunner> self) {
    return self->GetRightTrimResult();
}
extern "C" void vtk_executable_runner_right_trim_result_on(vtkNew<vtkExecutableRunner> self) {
    return self->RightTrimResultOn();
}
extern "C" void vtk_executable_runner_right_trim_result_off(vtkNew<vtkExecutableRunner> self) {
    return self->RightTrimResultOff();
}
extern "C" void vtk_executable_runner_set_execute_in_system_shell(
    vtkNew<vtkExecutableRunner> self, bool _arg
) {
    return self->SetExecuteInSystemShell(_arg);
}
extern "C" bool vtk_executable_runner_get_execute_in_system_shell(
    vtkNew<vtkExecutableRunner> self
) {
    return self->GetExecuteInSystemShell();
}
extern "C" void vtk_executable_runner_execute_in_system_shell_on(vtkNew<vtkExecutableRunner> self) {
    return self->ExecuteInSystemShellOn();
}
extern "C" void vtk_executable_runner_execute_in_system_shell_off(
    vtkNew<vtkExecutableRunner> self
) {
    return self->ExecuteInSystemShellOff();
}
extern "C" void vtk_executable_runner_clear_arguments(vtkNew<vtkExecutableRunner> self) {
    return self->ClearArguments();
}
extern "C" long long vtk_executable_runner_get_number_of_arguments(
    vtkNew<vtkExecutableRunner> self
) {
    return self->GetNumberOfArguments();
}
extern "C" int vtk_executable_runner_get_return_value(vtkNew<vtkExecutableRunner> self) {
    return self->GetReturnValue();
}
extern "C" vtkNew<vtkServerSocket> vtkServerSocket_new() {
    return vtkNew<vtkServerSocket>();
}
extern "C" void vtkServerSocket_destructor(vtkNew<vtkServerSocket> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkServerSocket_get_ptr(vtkNew<vtkServerSocket> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_server_socket_create_server_c_int(vtkNew<vtkServerSocket> self, int port) {
    return self->CreateServer(port);
}
extern "C" int vtk_server_socket_get_server_port(vtkNew<vtkServerSocket> self) {
    return self->GetServerPort();
}
extern "C" vtkNew<vtkSocketCollection> vtkSocketCollection_new() {
    return vtkNew<vtkSocketCollection>();
}
extern "C" void vtkSocketCollection_destructor(vtkNew<vtkSocketCollection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSocketCollection_get_ptr(vtkNew<vtkSocketCollection> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_socket_collection_select_sockets(
    vtkNew<vtkSocketCollection> self, unsigned long msec
) {
    return self->SelectSockets(msec);
}
extern "C" vtkNew<vtkTimerLog> vtkTimerLog_new() {
    return vtkNew<vtkTimerLog>();
}
extern "C" void vtkTimerLog_destructor(vtkNew<vtkTimerLog> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTimerLog_get_ptr(vtkNew<vtkTimerLog> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_timer_log_set_logging(vtkNew<vtkTimerLog> self, int v) {
    return self->SetLogging(v);
}
extern "C" int vtk_timer_log_get_logging(vtkNew<vtkTimerLog> self) {
    return self->GetLogging();
}
extern "C" void vtk_timer_log_logging_on(vtkNew<vtkTimerLog> self) {
    return self->LoggingOn();
}
extern "C" void vtk_timer_log_logging_off(vtkNew<vtkTimerLog> self) {
    return self->LoggingOff();
}
extern "C" void vtk_timer_log_set_max_entries(vtkNew<vtkTimerLog> self, int a) {
    return self->SetMaxEntries(a);
}
extern "C" int vtk_timer_log_get_max_entries(vtkNew<vtkTimerLog> self) {
    return self->GetMaxEntries();
}
extern "C" int vtk_timer_log_get_number_of_events(vtkNew<vtkTimerLog> self) {
    return self->GetNumberOfEvents();
}
extern "C" int vtk_timer_log_get_event_indent(vtkNew<vtkTimerLog> self, int i) {
    return self->GetEventIndent(i);
}
extern "C" double vtk_timer_log_get_event_wall_time(vtkNew<vtkTimerLog> self, int i) {
    return self->GetEventWallTime(i);
}
extern "C" void vtk_timer_log_reset_log(vtkNew<vtkTimerLog> self) {
    return self->ResetLog();
}
extern "C" void vtk_timer_log_cleanup_log(vtkNew<vtkTimerLog> self) {
    return self->CleanupLog();
}
extern "C" double vtk_timer_log_get_universal_time(vtkNew<vtkTimerLog> self) {
    return self->GetUniversalTime();
}
extern "C" double vtk_timer_log_get_cpu_time(vtkNew<vtkTimerLog> self) {
    return self->GetCPUTime();
}
extern "C" void vtk_timer_log_start_timer(vtkNew<vtkTimerLog> self) {
    return self->StartTimer();
}
extern "C" void vtk_timer_log_stop_timer(vtkNew<vtkTimerLog> self) {
    return self->StopTimer();
}
extern "C" double vtk_timer_log_get_elapsed_time(vtkNew<vtkTimerLog> self) {
    return self->GetElapsedTime();
}
