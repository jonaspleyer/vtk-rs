cmake_minimum_required(VERSION 3.12)

project(vtkrs)

find_package(VTK COMPONENTS
  CommonColor
  CommonCore
  CommonDataModel
  FiltersSources
  InteractionStyle
  RenderingContextOpenGL2
  RenderingCore
  RenderingFreeType
  RenderingGL2PSOpenGL2
  RenderingOpenGL2
)

if (NOT VTK_FOUND)
    message(FATAL_ERROR "Vtk: Unable to find the VTK build folder.")
endif()

# Prevent a "command line is too long" failure in Windows.
set(CMAKE_NINJA_FORCE_RESPONSE_FILE "ON" CACHE BOOL "Force Ninja to use response files.")

add_library(vtkrs STATIC
   ${PROJECT_SOURCE_DIR}/include/cxx.h
   ${PROJECT_SOURCE_DIR}/include/vtk_algorithm.h
   ${PROJECT_SOURCE_DIR}/include/vtk_data_object.h
   ${PROJECT_SOURCE_DIR}/include/vtk_named_colors.h
   ${PROJECT_SOURCE_DIR}/include/vtk_object.h
   ${PROJECT_SOURCE_DIR}/include/vtk_object_base.h
   ${PROJECT_SOURCE_DIR}/include/vtk_object_base.rs.h
   ${PROJECT_SOURCE_DIR}/include/vtk_poly_data.h
   ${PROJECT_SOURCE_DIR}/include/vtk_poly_data_mapper.h
   ${PROJECT_SOURCE_DIR}/include/vtk_sphere.h
   ${PROJECT_SOURCE_DIR}/include/vtk_sphere_source.h
)

if (VTK094)
    target_compile_definitions(vtkrs PUBLIC -DVTK094=1)
endif()

include_directories(${PROJECT_SOURCE_DIR}/include/)

target_sources(vtkrs
    PRIVATE
        ${PROJECT_SOURCE_DIR}/src/vtk_algorithm.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_data_object.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_named_colors.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_object.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_object_base.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_poly_data.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_poly_data_mapper.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_sphere.cpp
        ${PROJECT_SOURCE_DIR}/src/vtk_sphere_source.cpp
)
set_target_properties(vtkrs PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(vtkrs PRIVATE ${VTK_LIBRARIES})

install(TARGETS vtkrs DESTINATION .)

vtk_module_autoinit(
  TARGETS vtkrs
  MODULES ${VTK_LIBRARIES}
)

# add_executable(main main.cpp vtkPolyDataMapper.cpp)
# set_target_properties(main PROPERTIES LINKER_LANGUAGE CXX)
# target_link_libraries(main PRIVATE ${VTK_LIBRARIES})
