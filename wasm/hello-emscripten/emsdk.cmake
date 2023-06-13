set(EMSDK_VERSION 3.1.3)

include(FetchContent)
FetchContent_Declare(
    emsdk
    GIT_REPOSITORY https://github.com/emscripten-core/emsdk.git
    GIT_TAG ${EMSDK_VERSION}
    GIT_SHALLOW TRUE
    GIT_PROGRESS TRUE
)
FetchContent_GetProperties(emsdk)

if(NOT emsdk_POPULATED)
    FetchContent_Populate(emsdk)
endif()

set(EMSDK_DIR ${emsdk_SOURCE_DIR})

execute_process(COMMAND ${EMSDK_DIR}/emsdk install ${EMSDK_VERSION})
execute_process(COMMAND ${EMSDK_DIR}/emsdk activate ${EMSDK_VERSION})

set(CMAKE_TOOLCHAIN_FILE ${EMSDK_DIR}/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake)
