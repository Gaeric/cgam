# cmake version
cmake_minimum_required(VERSION 3.13)

# 工程名称
project(rtow)

# 设置
set(CMAKE_CXX_STANDARD 11)

# 编译源码生成目标
add_executable(rtow main.cc)
