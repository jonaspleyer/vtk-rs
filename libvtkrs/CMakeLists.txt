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
   ${PROJECT_SOURCE_DIR}/include/sphere.h
   ${PROJECT_SOURCE_DIR}/include/sphere_source.h
   ${PROJECT_SOURCE_DIR}/include/named_colors.h
   ${PROJECT_SOURCE_DIR}/include/cxx.h
   # ${PROJECT_SOURCE_DIR}/include/vtkPolyDataMapper.h
   ${PROJECT_SOURCE_DIR}/include/cxx_sphere.h
   ${PROJECT_SOURCE_DIR}/include/poly_data_mapper.h
)

include_directories(${PROJECT_SOURCE_DIR}/include/)

target_sources(vtkrs
    PRIVATE
        ${PROJECT_SOURCE_DIR}/src/sphere.cpp
        ${PROJECT_SOURCE_DIR}/src/sphere_source.cpp
        ${PROJECT_SOURCE_DIR}/src/named_colors.cpp
        ${PROJECT_SOURCE_DIR}/src/cxx_sphere.cpp
        ${PROJECT_SOURCE_DIR}/src/poly_data_mapper.cpp
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
