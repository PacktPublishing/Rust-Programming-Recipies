cmake_minimum_required(VERSION 3.0)
project(LibBadMath C)
add_library(badmath STATIC badmath.c)
install(TARGETS badmath DESTINATION .) 
