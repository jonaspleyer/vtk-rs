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

// Declare exported functions
extern "C" vtkClientSocket * vtkClientSocket_new () ;
extern "C" void vtkClientSocket_destructor (vtkClientSocket * sself) ;
extern "C" int vtk_client_socket_connect_to_server(vtkClientSocket* sself, const char* hostname, int port);
extern "C" bool vtk_client_socket_get_connecting_side(vtkClientSocket* sself);
extern "C" vtkDirectory * vtkDirectory_new () ;
extern "C" void vtkDirectory_destructor (vtkDirectory * sself) ;
extern "C" int vtk_directory_open(vtkDirectory* sself, const char* dir);
extern "C" long long vtk_directory_get_number_of_files(vtkDirectory* sself);
extern "C" const char* vtk_directory_get_file(vtkDirectory* sself, long long index);
extern "C" int vtk_directory_file_is_directory(vtkDirectory* sself, const char* name);
extern "C" int vtk_directory_make_directory(vtkDirectory* sself, const char* dir);
extern "C" int vtk_directory_delete_directory(vtkDirectory* sself, const char* dir);
extern "C" int vtk_directory_rename(vtkDirectory* sself, const char* oldname, const char* newname);
extern "C" vtkExecutableRunner * vtkExecutableRunner_new () ;
extern "C" void vtkExecutableRunner_destructor (vtkExecutableRunner * sself) ;
extern "C" void vtk_executable_runner_execute(vtkExecutableRunner* sself);
extern "C" void vtk_executable_runner_set_timeout(vtkExecutableRunner* sself, double _arg);
extern "C" double vtk_executable_runner_get_timeout(vtkExecutableRunner* sself);
extern "C" void vtk_executable_runner_set_right_trim_result(vtkExecutableRunner* sself, bool _arg);
extern "C" bool vtk_executable_runner_get_right_trim_result(vtkExecutableRunner* sself);
extern "C" void vtk_executable_runner_right_trim_result_on(vtkExecutableRunner* sself);
extern "C" void vtk_executable_runner_right_trim_result_off(vtkExecutableRunner* sself);
extern "C" const char* vtk_executable_runner_get_command(vtkExecutableRunner* sself);
extern "C" void vtk_executable_runner_set_command(vtkExecutableRunner* sself, const char* arg);
extern "C" const char* vtk_executable_runner_get_std_out(vtkExecutableRunner* sself);
extern "C" const char* vtk_executable_runner_get_std_err(vtkExecutableRunner* sself);
extern "C" int vtk_executable_runner_get_return_value(vtkExecutableRunner* sself);
extern "C" vtkServerSocket * vtkServerSocket_new () ;
extern "C" void vtkServerSocket_destructor (vtkServerSocket * sself) ;
extern "C" int vtk_server_socket_create_server(vtkServerSocket* sself, int port);
extern "C" int vtk_server_socket_get_server_port(vtkServerSocket* sself);
extern "C" vtkSocketCollection * vtkSocketCollection_new () ;
extern "C" void vtkSocketCollection_destructor (vtkSocketCollection * sself) ;
extern "C" int vtk_socket_collection_select_sockets(vtkSocketCollection* sself, unsigned long msec);
extern "C" vtkThreadMessager * vtkThreadMessager_new () ;
extern "C" void vtkThreadMessager_destructor (vtkThreadMessager * sself) ;
extern "C" void vtk_thread_messager_wait_for_message(vtkThreadMessager* sself);
extern "C" void vtk_thread_messager_send_wake_message(vtkThreadMessager* sself);
extern "C" void vtk_thread_messager_enable_wait_for_receiver(vtkThreadMessager* sself);
extern "C" void vtk_thread_messager_disable_wait_for_receiver(vtkThreadMessager* sself);
extern "C" void vtk_thread_messager_wait_for_receiver(vtkThreadMessager* sself);
extern "C" vtkTimerLog * vtkTimerLog_new () ;
extern "C" void vtkTimerLog_destructor (vtkTimerLog * sself) ;
extern "C" void vtk_timer_log_set_logging(vtkTimerLog* sself, int v);
extern "C" int vtk_timer_log_get_logging(vtkTimerLog* sself);
extern "C" void vtk_timer_log_logging_on(vtkTimerLog* sself);
extern "C" void vtk_timer_log_logging_off(vtkTimerLog* sself);
extern "C" void vtk_timer_log_set_max_entries(vtkTimerLog* sself, int a);
extern "C" int vtk_timer_log_get_max_entries(vtkTimerLog* sself);
extern "C" void vtk_timer_log_dump_log(vtkTimerLog* sself, const char* filename);
extern "C" void vtk_timer_log_mark_start_event(vtkTimerLog* sself, const char* EventString);
extern "C" void vtk_timer_log_mark_end_event(vtkTimerLog* sself, const char* EventString);
extern "C" void vtk_timer_log_insert_timed_event(vtkTimerLog* sself, const char* EventString, double time, int cpuTicks);
extern "C" int vtk_timer_log_get_number_of_events(vtkTimerLog* sself);
extern "C" int vtk_timer_log_get_event_indent(vtkTimerLog* sself, int i);
extern "C" double vtk_timer_log_get_event_wall_time(vtkTimerLog* sself, int i);
extern "C" const char* vtk_timer_log_get_event_string(vtkTimerLog* sself, int i);
extern "C" void vtk_timer_log_mark_event(vtkTimerLog* sself, const char* EventString);
extern "C" void vtk_timer_log_reset_log(vtkTimerLog* sself);
extern "C" void vtk_timer_log_cleanup_log(vtkTimerLog* sself);
extern "C" double vtk_timer_log_get_universal_time(vtkTimerLog* sself);
extern "C" double vtk_timer_log_get_cpu_time(vtkTimerLog* sself);
extern "C" void vtk_timer_log_start_timer(vtkTimerLog* sself);
extern "C" void vtk_timer_log_stop_timer(vtkTimerLog* sself);
extern "C" double vtk_timer_log_get_elapsed_time(vtkTimerLog* sself);
