cmake_minimum_required(VERSION 3.10)
project("cmake example for Rust" LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)

set(TARGET wrapper)
add_library(${TARGET}
  STATIC
    wrapper.cc
)
install (TARGETS ${TARGET} DESTINATION .)
