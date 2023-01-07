cmake_minimum_required(VERSION 3.13)

include(SDK_PATH/pico_sdk_init.cmake)

project(PROJECT_NAME)

pico_sdk_init()

add_executable(PROJECT_NAME
    PROJECT_NAME.c
)

target_link_libraries(PROJECT_NAME pico_stdlib)

pico_add_extra_outputs(PROJECT_NAME)