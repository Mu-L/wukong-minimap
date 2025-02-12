#pragma once
#include <d3d12.h>
#include <string>
#include <unordered_map>
#include "ImGui/imgui.h"

class TextureManager
{
public:
    static TextureManager &Instance()
    {
        static TextureManager instance;
        return instance;
    }

    bool LoadTexture(ID3D12Device *device, const std::string &key, const std::string &path,
                     ID3D12DescriptorHeap *srvHeap, UINT descriptorIndex, ID3D12CommandQueue *cmdQueue);
    bool UpdateTexture(ID3D12Device *device, const std::string &key, const std::string &path, ID3D12CommandQueue *cmdQueue);
    void Release();
    ImTextureID GetTextureID(const std::string &key);

private:
    TextureManager() = default;
    std::unordered_map<std::string, ImTextureID> textureMap;       // key -> ImTextureID
    std::unordered_map<std::string, ID3D12Resource *> resourceMap; // key -> Resource
};
