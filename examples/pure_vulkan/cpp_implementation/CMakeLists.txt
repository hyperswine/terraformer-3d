cmake_minimum_required(VERSION 3.11b)

project(mainprogram)

set(TARGET_NAME mainprogram)

# RN only targets macos
if(APPLE)
    set(OS_LIBRARIES "-framework Cocoa" "-framework CoreVideo" "-framework IOKit" "-framework QuartzCore")
endif(APPLE)

include(FetchContent)

# fetch glfw3
FetchContent_Declare(
  glfw3
  GIT_REPOSITORY https://github.com/glfw/glfw.git
  GIT_TAG        45ce5ddd197d5c58f50fdd3296a5131c894e5527 # 3.3.7
)

find_package(glfw3 3.3 REQUIRED
    HINTS "$ENV{GLFW3_INSTALL_DIR}"
)

add_executable(${TARGET_NAME} main.cpp)
target_include_directories(${TARGET_NAME} PUBLIC $ENV{GLFW3_INCLUDE_DIR})
target_link_libraries(${TARGET_NAME} glfw ${OS_LIBRARIES})
