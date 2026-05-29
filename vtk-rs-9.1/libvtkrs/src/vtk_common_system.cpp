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
#include<vtkThreadMessager.h>
#include<vtkTimerLog.h>
#include<vtkTimerLog.h>
#include<vtkTimerLog.h>

// Implement declared functions
extern "C" vtkClientSocket * vtkClientSocket_new () {return vtkClientSocket :: New () ;}
extern "C" void vtkClientSocket_destructor (vtkClientSocket * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_client_socket_connect_to_server(vtkClientSocket* sself, const char* hostname, int port) { return sself->ConnectToServer(hostname, port); }
extern "C" bool vtk_client_socket_get_connecting_side(vtkClientSocket* sself) { return sself->GetConnectingSide(); }
extern "C" vtkDirectory * vtkDirectory_new () {return vtkDirectory :: New () ;}
extern "C" void vtkDirectory_destructor (vtkDirectory * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_directory_open(vtkDirectory* sself, const char* dir) { return sself->Open(dir); }
extern "C" long long vtk_directory_get_number_of_files(vtkDirectory* sself) { return sself->GetNumberOfFiles(); }
extern "C" const char* vtk_directory_get_file(vtkDirectory* sself, long long index) { return sself->GetFile(index); }
extern "C" int vtk_directory_file_is_directory(vtkDirectory* sself, const char* name) { return sself->FileIsDirectory(name); }
extern "C" int vtk_directory_make_directory(vtkDirectory* sself, const char* dir) { return sself->MakeDirectory(dir); }
extern "C" int vtk_directory_delete_directory(vtkDirectory* sself, const char* dir) { return sself->DeleteDirectory(dir); }
extern "C" int vtk_directory_rename(vtkDirectory* sself, const char* oldname, const char* newname) { return sself->Rename(oldname, newname); }
extern "C" vtkExecutableRunner * vtkExecutableRunner_new () {return vtkExecutableRunner :: New () ;}
extern "C" void vtkExecutableRunner_destructor (vtkExecutableRunner * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_executable_runner_execute(vtkExecutableRunner* sself) { sself->Execute(); }
extern "C" void vtk_executable_runner_set_timeout(vtkExecutableRunner* sself, double _arg) { sself->SetTimeout(_arg); }
extern "C" double vtk_executable_runner_get_timeout(vtkExecutableRunner* sself) { return sself->GetTimeout(); }
extern "C" void vtk_executable_runner_set_right_trim_result(vtkExecutableRunner* sself, bool _arg) { sself->SetRightTrimResult(_arg); }
extern "C" bool vtk_executable_runner_get_right_trim_result(vtkExecutableRunner* sself) { return sself->GetRightTrimResult(); }
extern "C" void vtk_executable_runner_right_trim_result_on(vtkExecutableRunner* sself) { sself->RightTrimResultOn(); }
extern "C" void vtk_executable_runner_right_trim_result_off(vtkExecutableRunner* sself) { sself->RightTrimResultOff(); }
extern "C" const char* vtk_executable_runner_get_command(vtkExecutableRunner* sself) { return sself->GetCommand(); }
extern "C" void vtk_executable_runner_set_command(vtkExecutableRunner* sself, const char* arg) { sself->SetCommand(arg); }
extern "C" const char* vtk_executable_runner_get_std_out(vtkExecutableRunner* sself) { return sself->GetStdOut(); }
extern "C" const char* vtk_executable_runner_get_std_err(vtkExecutableRunner* sself) { return sself->GetStdErr(); }
extern "C" int vtk_executable_runner_get_return_value(vtkExecutableRunner* sself) { return sself->GetReturnValue(); }
extern "C" vtkServerSocket * vtkServerSocket_new () {return vtkServerSocket :: New () ;}
extern "C" void vtkServerSocket_destructor (vtkServerSocket * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_server_socket_create_server(vtkServerSocket* sself, int port) { return sself->CreateServer(port); }
extern "C" int vtk_server_socket_get_server_port(vtkServerSocket* sself) { return sself->GetServerPort(); }
extern "C" vtkSocketCollection * vtkSocketCollection_new () {return vtkSocketCollection :: New () ;}
extern "C" void vtkSocketCollection_destructor (vtkSocketCollection * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_socket_collection_select_sockets(vtkSocketCollection* sself, unsigned long msec) { return sself->SelectSockets(msec); }
extern "C" vtkThreadMessager * vtkThreadMessager_new () {return vtkThreadMessager :: New () ;}
extern "C" void vtkThreadMessager_destructor (vtkThreadMessager * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_thread_messager_wait_for_message(vtkThreadMessager* sself) { sself->WaitForMessage(); }
extern "C" void vtk_thread_messager_send_wake_message(vtkThreadMessager* sself) { sself->SendWakeMessage(); }
extern "C" void vtk_thread_messager_enable_wait_for_receiver(vtkThreadMessager* sself) { sself->EnableWaitForReceiver(); }
extern "C" void vtk_thread_messager_disable_wait_for_receiver(vtkThreadMessager* sself) { sself->DisableWaitForReceiver(); }
extern "C" void vtk_thread_messager_wait_for_receiver(vtkThreadMessager* sself) { sself->WaitForReceiver(); }
extern "C" vtkTimerLog * vtkTimerLog_new () {return vtkTimerLog :: New () ;}
extern "C" void vtkTimerLog_destructor (vtkTimerLog * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_timer_log_set_logging(vtkTimerLog* sself, int v) { sself->SetLogging(v); }
extern "C" int vtk_timer_log_get_logging(vtkTimerLog* sself) { return sself->GetLogging(); }
extern "C" void vtk_timer_log_logging_on(vtkTimerLog* sself) { sself->LoggingOn(); }
extern "C" void vtk_timer_log_logging_off(vtkTimerLog* sself) { sself->LoggingOff(); }
extern "C" void vtk_timer_log_set_max_entries(vtkTimerLog* sself, int a) { sself->SetMaxEntries(a); }
extern "C" int vtk_timer_log_get_max_entries(vtkTimerLog* sself) { return sself->GetMaxEntries(); }
extern "C" void vtk_timer_log_dump_log(vtkTimerLog* sself, const char* filename) { sself->DumpLog(filename); }
extern "C" void vtk_timer_log_mark_start_event(vtkTimerLog* sself, const char* EventString) { sself->MarkStartEvent(EventString); }
extern "C" void vtk_timer_log_mark_end_event(vtkTimerLog* sself, const char* EventString) { sself->MarkEndEvent(EventString); }
extern "C" void vtk_timer_log_insert_timed_event(vtkTimerLog* sself, const char* EventString, double time, int cpuTicks) { sself->InsertTimedEvent(EventString, time, cpuTicks); }
extern "C" int vtk_timer_log_get_number_of_events(vtkTimerLog* sself) { return sself->GetNumberOfEvents(); }
extern "C" int vtk_timer_log_get_event_indent(vtkTimerLog* sself, int i) { return sself->GetEventIndent(i); }
extern "C" double vtk_timer_log_get_event_wall_time(vtkTimerLog* sself, int i) { return sself->GetEventWallTime(i); }
extern "C" const char* vtk_timer_log_get_event_string(vtkTimerLog* sself, int i) { return sself->GetEventString(i); }
extern "C" void vtk_timer_log_mark_event(vtkTimerLog* sself, const char* EventString) { sself->MarkEvent(EventString); }
extern "C" void vtk_timer_log_reset_log(vtkTimerLog* sself) { sself->ResetLog(); }
extern "C" void vtk_timer_log_cleanup_log(vtkTimerLog* sself) { sself->CleanupLog(); }
extern "C" double vtk_timer_log_get_universal_time(vtkTimerLog* sself) { return sself->GetUniversalTime(); }
extern "C" double vtk_timer_log_get_cpu_time(vtkTimerLog* sself) { return sself->GetCPUTime(); }
extern "C" void vtk_timer_log_start_timer(vtkTimerLog* sself) { sself->StartTimer(); }
extern "C" void vtk_timer_log_stop_timer(vtkTimerLog* sself) { sself->StopTimer(); }
extern "C" double vtk_timer_log_get_elapsed_time(vtkTimerLog* sself) { return sself->GetElapsedTime(); }
