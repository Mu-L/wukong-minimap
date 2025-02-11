# wukong-minimap-cpp

## Getting started

To get started, run the following commands from your project directory:

```sh
curl https://raw.githubusercontent.com/build-cpp/cmkr/main/cmake/cmkr.cmake -o cmkr.cmake
cmake -P cmkr.cmake
```

After the bootstrapping process finishes, customize [`cmake.toml`](https://build-cpp.github.io/cmkr/cmake-toml) for your project and run CMake:

```sh
cmake -B build -G "Visual Studio 17 2022" -A x64

cmake --build build --config Debug    # 构建 Debug
cmake --build build --config Release  # 构建 Release
```

Once bootstrapped, `cmkr` does not introduce extra steps to your workflow. After modifying `cmake.toml` you simply build/configure your CMake project and `cmkr` will automatically regenerate `CMakeLists.txt` when necessary.

<sub>**Note**: The `cmake.toml` project file, generated `CMakeLists.txt` and `cmkr.cmake` bootstrapping script are all intended to be added to source control.</sub>

In CI environments the `cmkr` bootstrapping process is skipped, so there is no additional overhead in your pipelines.
