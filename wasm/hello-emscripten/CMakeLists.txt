cmake_minimum_required(VERSION 3.20)

include(emsdk.cmake)

project(hello-emscripten)

add_executable(${PROJECT_NAME}
  src/array.cpp
  src/callback.cpp
  src/error.cpp
  src/string.cpp
)

target_include_directories(${PROJECT_NAME} PRIVATE ${HELLO_DIR}/include)
target_link_directories(${PROJECT_NAME} PRIVATE ${HELLO_DIR}/lib)
target_link_libraries(${PROJECT_NAME} hello_cpp)

set(OUTPUT_NAME index)

set_target_properties(${PROJECT_NAME} PROPERTIES
  OUTPUT_NAME ${OUTPUT_NAME}
  CXX_STANDARD 17
  LINK_FLAGS "\
  -s MODULARIZE \
  -s EXPORT_ES6 \
  -s NO_DISABLE_EXCEPTION_CATCHING \
  --bind \
  "
)

install(
  FILES
  ${CMAKE_CURRENT_BINARY_DIR}/${OUTPUT_NAME}.js
  ${CMAKE_CURRENT_BINARY_DIR}/${OUTPUT_NAME}.wasm
  DESTINATION .
)
