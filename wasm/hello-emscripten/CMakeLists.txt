cmake_minimum_required(VERSION 3.20)

include(emscripten.cmake)

project(hello-emscripten)

add_executable(${PROJECT_NAME}
  src/lib.cpp
)

target_include_directories(${PROJECT_NAME} PRIVATE ${HELLO_CPP_INCLUDE_DIR})
target_link_directories(${PROJECT_NAME} PRIVATE ${HELLO_CPP_LIB_DIR})
target_link_libraries(${PROJECT_NAME} hello_cpp)

set(OUTPUT_NAME index)

set_target_properties(${PROJECT_NAME} PROPERTIES
  OUTPUT_NAME ${OUTPUT_NAME}
  CXX_STANDARD 17
  LINK_FLAGS "\
    -s MODULARIZE=1 \
    -s EXPORT_ES6=1 \
    -l embind \
  "
)

install(
  FILES
    ${CMAKE_CURRENT_BINARY_DIR}/${OUTPUT_NAME}.js
    ${CMAKE_CURRENT_BINARY_DIR}/${OUTPUT_NAME}.wasm
  DESTINATION .
)
