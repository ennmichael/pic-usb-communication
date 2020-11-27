cmake_minimum_required(VERSION 3.16)
project(pic_playground C)

if (NOT ${CMAKE_C_COMPILER} MATCHES ".*xc8-cc.*")
    message(ERROR "The project must be compiled with xc8-cc. Use -DCMAKE_C_COMPILER=/path/to/xc8-cc.")
endif ()

set(CMAKE_C_COMPILER_WORKS 1)

add_executable(pic_playground main.c)
target_compile_options(pic_playground PUBLIC "-mcpu=pic16f1937")
target_compile_options(pic_playground PUBLIC "-std=c99")
target_link_options(pic_playground PUBLIC "-mcpu=pic16f1937")

add_custom_command(TARGET pic_playground PRE_LINK COMMAND cp main.c.p1 main.c.o WORKING_DIRECTORY CMakeFiles/pic_playground.dir/)
