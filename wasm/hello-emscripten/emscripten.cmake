include(FetchContent)

FetchContent_Declare(
    emsdk
    GIT_REPOSITORY  https://github.com/emscripten-core/emsdk.git
    GIT_TAG         3.1.3
    GIT_SHALLOW     TRUE
    GIT_PROGRESS    TRUE
)
FetchContent_GetProperties(emsdk)
if(NOT emsdk_POPULATED)
    FetchContent_Populate(emsdk)
endif()

set(EMSDK_VERSION 3.1.3)
set(EMSDK_COMMAND ${emsdk_SOURCE_DIR}/emsdk)

execute_process(
    COMMAND ${EMSDK_COMMAND} install ${EMSDK_VERSION}
    WORKING_DIRECTORY ${emsdk_SOURCE_DIR}
)
execute_process(
    COMMAND ${EMSDK_COMMAND} activate ${EMSDK_VERSION}
    WORKING_DIRECTORY ${emsdk_SOURCE_DIR}
)

set(CMAKE_TOOLCHAIN_FILE "${emsdk_SOURCE_DIR}/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake" CACHE FILEPATH "emscripten cmake toolchain" FORCE)
