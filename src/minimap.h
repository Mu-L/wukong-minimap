#pragma once
#include "stdafx.h"
#include <string>
#include <vector>
#include "json.hpp"

struct Range
{
  std::array<float, 3> start;
  std::array<float, 3> end;
};

struct Point
{
  std::string name;
  int icon;
  std::string key;
  float x;
  float y;
  float z;
};

struct MapInfo
{
  std::string level;
  std::string key;
  std::string url;
  float scale;
  Range range;
  std::vector<Range> areas;
  std::vector<Point> points;
};

// JSON 序列化支持
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Range, start, end)
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Point, name, icon, key, x, y, z)
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(MapInfo, level, key, url, scale, range, areas, points)

struct PlayerInfo
{
  float x;
  float y;
  float z;
  float angle;
  uint8_t bIsLocalViewTarget;
  std::string level;
};

namespace Minimap
{
  struct SDKMgr
  {
    std::map<std::string, void *> data;

    bool registPtr(std::string key, void *ptr)
    {
      /*DEBUG_PRINT("[registered] %s:%llx", key.c_str(), ptr);*/
      data[key] = ptr;
      return true;
    }

    template <typename T>
    T get(std::string key)
    {
      auto it = data.find(key);
      if (it != data.end())
      {
        return reinterpret_cast<T>(it->second);
      }
      return nullptr;
    }
  };

  inline static SDKMgr sdkMgr;
  void init();
  void draw();
  PlayerInfo getPlayerInfo();
}