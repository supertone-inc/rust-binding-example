cmake_minimum_required(VERSION 3.20)
include(emsdk.cmake)
project(hello-emscripten)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(${PROJECT_NAME}
  src/array.cpp
  src/callback.cpp
  src/error.cpp
  src/string.cpp
)
set_target_properties(${PROJECT_NAME} PROPERTIES
  OUTPUT_NAME index
  CXX_STANDARD 17
  LINK_FLAGS "\
    -s MODULARIZE \
    -s EXPORT_ES6 \
    -s NO_DISABLE_EXCEPTION_CATCHING \
    --bind \
    "
)
target_include_directories(${PROJECT_NAME} PRIVATE include)
target_link_libraries(${PROJECT_NAME} ${CMAKE_CURRENT_LIST_DIR}/lib/libhello_emscripten.a)

install(
  FILES
  ${CMAKE_CURRENT_BINARY_DIR}/index.js
  ${CMAKE_CURRENT_BINARY_DIR}/index.wasm
  DESTINATION .
)
