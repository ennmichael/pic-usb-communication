cmake_minimum_required(VERSION 3.16)
project(device C)

if (NOT ${CMAKE_C_COMPILER} MATCHES ".*xc8-cc.*")
    message(FATAL_ERROR "The project must be compiled with xc8-cc. Use -DCMAKE_C_COMPILER=/path/to/xc8-cc.")
endif ()

set(SOURCES "main.c" "device.c" "led.c")

add_executable(device ${SOURCES})
target_compile_options(device PUBLIC "-mcpu=pic16f1937")
target_compile_options(device PUBLIC "-std=c99")
target_compile_options(device PUBLIC "-mreserve=ram@0x320:0x32F")
target_link_options(device PUBLIC "-mcpu=pic16f1937")

foreach (FILE ${SOURCES})
    add_custom_command(TARGET device PRE_LINK COMMAND cp ${FILE}.p1 ${FILE}.o WORKING_DIRECTORY CMakeFiles/device.dir/)
endforeach()
