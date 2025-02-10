#pragma once
#include <string>

void DrawInit();
bool LoadTextureFromFile(const std::string &key, const char *filename);
bool UnloadTextureFormFile(const std::string &key, const char *filename);