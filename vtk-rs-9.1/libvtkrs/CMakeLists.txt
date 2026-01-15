cmake_minimum_required(VERSION 3.12)

project(vtkrs)

find_package(VTK COMPONENTS
    CommonArchive
    CommonColor
    CommonComputationalGeometry
    CommonCore
    CommonDataModel
    CommonExecutionModel
    CommonMath
    CommonMisc
    CommonPython
    CommonSystem
    CommonTransforms
)

if (NOT VTK_FOUND)
    message(FATAL_ERROR "Vtk: Unable to find the VTK build folder.")
endif()

# Prevent a "command line is too long" failure in Windows.
set(CMAKE_NINJA_FORCE_RESPONSE_FILE "ON" CACHE BOOL "Force Ninja to use response files.")

add_library(vtkrs STATIC
    ${PROJECT_SOURCE_DIR}/include/vtk_common_archive.h
    ${PROJECT_SOURCE_DIR}/include/vtk_common_color.h
    ${PROJECT_SOURCE_DIR}/include/vtk_common_computational_geometry.h
    ${PROJECT_SOURCE_DIR}/include/vtk_common_core.h
    ${PROJECT_SOURCE_DIR}/include/vtk_common_data_model.h
    ${PROJECT_SOURCE_DIR}/include/vtk_common_execution_model.h
    ${PROJECT_SOURCE_DIR}/include/vtk_common_math.h
    ${PROJECT_SOURCE_DIR}/include/vtk_common_misc.h
    ${PROJECT_SOURCE_DIR}/include/vtk_common_python.h
    ${PROJECT_SOURCE_DIR}/include/vtk_common_system.h
    ${PROJECT_SOURCE_DIR}/include/vtk_common_transforms.h
)

if (VTK094)
    target_compile_definitions(vtkrs PUBLIC -DVTK094=1)
endif()

include_directories(${PROJECT_SOURCE_DIR}/include/)
target_sources(vtkrs
     PRIVATE
        ${PROJECT_SOURCE_DIR}/src/vtk_common_archive.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_common_color.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_common_computational_geometry.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_common_core.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_common_data_model.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_common_execution_model.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_common_math.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_common_misc.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_common_python.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_common_system.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_common_transforms.cpp
)

set_target_properties(vtkrs PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(vtkrs PRIVATE ${VTK_LIBRARIES})

install(TARGETS vtkrs DESTINATION .)

vtk_module_autoinit(
  TARGETS vtkrs
  MODULES ${VTK_LIBRARIES}
)
