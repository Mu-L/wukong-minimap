#include <d3d12.h>
#include "texture_manager.h"
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

bool TextureManager::LoadTexture(ID3D12Device *device, const std::string &key, const std::string &path, ID3D12DescriptorHeap *srvHeap, UINT descriptorIndex, ID3D12CommandQueue *cmdQueue)
{
    // 检查是否已存在
    if (textureMap.find(key) != textureMap.end())
    {
        return false;
    }

    int width, height, channels;
    unsigned char *imageData = stbi_load(path.c_str(), &width, &height, &channels, 4);
    if (!imageData)
        return false;

    // 创建纹理资源
    D3D12_HEAP_PROPERTIES props = {};
    props.Type = D3D12_HEAP_TYPE_DEFAULT;
    props.CPUPageProperty = D3D12_CPU_PAGE_PROPERTY_UNKNOWN;
    props.MemoryPoolPreference = D3D12_MEMORY_POOL_UNKNOWN;

    D3D12_RESOURCE_DESC desc = {};
    desc.Dimension = D3D12_RESOURCE_DIMENSION_TEXTURE2D;
    desc.Width = width;
    desc.Height = height;
    desc.DepthOrArraySize = 1;
    desc.MipLevels = 1;
    desc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    desc.SampleDesc.Count = 1;
    desc.Layout = D3D12_TEXTURE_LAYOUT_UNKNOWN;
    desc.Flags = D3D12_RESOURCE_FLAG_NONE;

    ID3D12Resource *textureResource = nullptr;
    HRESULT hr = device->CreateCommittedResource(&props, D3D12_HEAP_FLAG_NONE, &desc,
                                                 D3D12_RESOURCE_STATE_COPY_DEST, nullptr, IID_PPV_ARGS(&textureResource));
    if (FAILED(hr))
    {
        stbi_image_free(imageData);
        return false;
    }

    // 计算对齐的上传缓冲区大小
    UINT uploadPitch = (width * 4 + D3D12_TEXTURE_DATA_PITCH_ALIGNMENT - 1u) & ~(D3D12_TEXTURE_DATA_PITCH_ALIGNMENT - 1u);
    UINT uploadSize = height * uploadPitch;

    // 创建上传缓冲区
    props.Type = D3D12_HEAP_TYPE_UPLOAD;
    desc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
    desc.Width = uploadSize;
    desc.Height = 1;
    desc.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;
    desc.Format = DXGI_FORMAT_UNKNOWN;

    ID3D12Resource *uploadBuffer = nullptr;
    hr = device->CreateCommittedResource(&props, D3D12_HEAP_FLAG_NONE, &desc,
                                         D3D12_RESOURCE_STATE_GENERIC_READ, nullptr, IID_PPV_ARGS(&uploadBuffer));
    if (FAILED(hr))
    {
        stbi_image_free(imageData);
        textureResource->Release();
        return false;
    }

    // 复制数据到上传缓冲区
    void *mapped = nullptr;
    D3D12_RANGE range = {0, uploadSize};
    uploadBuffer->Map(0, &range, &mapped);
    for (int y = 0; y < height; y++)
    {
        memcpy((void *)((uintptr_t)mapped + y * uploadPitch),
               imageData + y * width * 4,
               width * 4);
    }
    uploadBuffer->Unmap(0, &range);

    // 设置复制区域
    D3D12_TEXTURE_COPY_LOCATION src = {};
    src.pResource = uploadBuffer;
    src.Type = D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT;
    src.PlacedFootprint.Footprint.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    src.PlacedFootprint.Footprint.Width = width;
    src.PlacedFootprint.Footprint.Height = height;
    src.PlacedFootprint.Footprint.Depth = 1;
    src.PlacedFootprint.Footprint.RowPitch = uploadPitch;

    D3D12_TEXTURE_COPY_LOCATION dst = {};
    dst.pResource = textureResource;
    dst.Type = D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX;
    dst.SubresourceIndex = 0;

    // 创建命令列表并执行复制
    ID3D12CommandAllocator *cmdAlloc;
    ID3D12GraphicsCommandList *cmdList;
    device->CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT, IID_PPV_ARGS(&cmdAlloc));
    device->CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, cmdAlloc, nullptr, IID_PPV_ARGS(&cmdList));

    cmdList->CopyTextureRegion(&dst, 0, 0, 0, &src, nullptr);

    // 转换资源状态
    D3D12_RESOURCE_BARRIER barrier = {};
    barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
    barrier.Flags = D3D12_RESOURCE_BARRIER_FLAG_NONE;
    barrier.Transition.pResource = textureResource;
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_COPY_DEST;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE;
    barrier.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
    cmdList->ResourceBarrier(1, &barrier);
    cmdList->Close();

    // 执行命令并等待完成
    ID3D12Fence *fence = nullptr;
    device->CreateFence(0, D3D12_FENCE_FLAG_NONE, IID_PPV_ARGS(&fence));
    HANDLE event = CreateEvent(nullptr, FALSE, FALSE, nullptr);

    cmdQueue->ExecuteCommandLists(1, (ID3D12CommandList **)&cmdList);
    cmdQueue->Signal(fence, 1);
    fence->SetEventOnCompletion(1, event);
    WaitForSingleObject(event, INFINITE);

    // 创建 SRV
    D3D12_SHADER_RESOURCE_VIEW_DESC srvDesc = {};
    srvDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    srvDesc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2D;
    srvDesc.Texture2D.MipLevels = 1;
    srvDesc.Shader4ComponentMapping = D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING;

    D3D12_CPU_DESCRIPTOR_HANDLE srvHandle = srvHeap->GetCPUDescriptorHandleForHeapStart();
    srvHandle.ptr += descriptorIndex * device->GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV);
    device->CreateShaderResourceView(textureResource, &srvDesc, srvHandle);

    // 清理临时资源
    stbi_image_free(imageData);
    uploadBuffer->Release();
    cmdList->Release();
    cmdAlloc->Release();
    CloseHandle(event);
    fence->Release();

    // 保存纹理资源和GPU句柄，使用key而不是path
    resourceMap[key] = textureResource;
    D3D12_GPU_DESCRIPTOR_HANDLE gpuHandle = srvHeap->GetGPUDescriptorHandleForHeapStart();
    gpuHandle.ptr += descriptorIndex * device->GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV);
    textureMap[key] = (ImTextureID)gpuHandle.ptr;
    return true;
}

bool TextureManager::UpdateTexture(ID3D12Device *device, const std::string &key, const std::string &path, ID3D12CommandQueue *cmdQueue)
{
    // 检查纹理是否存在
    auto textureIt = textureMap.find(key);
    auto resourceIt = resourceMap.find(key);
    if (textureIt == textureMap.end() || resourceIt == resourceMap.end())
    {
        return false;
    }

    // 加载新的图片数据
    int width, height, channels;
    unsigned char *imageData = stbi_load(path.c_str(), &width, &height, &channels, 4);
    if (!imageData)
        return false;

    // 创建上传缓冲区
    UINT uploadPitch = (width * 4 + D3D12_TEXTURE_DATA_PITCH_ALIGNMENT - 1u) & ~(D3D12_TEXTURE_DATA_PITCH_ALIGNMENT - 1u);
    UINT uploadSize = height * uploadPitch;

    D3D12_HEAP_PROPERTIES uploadProps = {};
    uploadProps.Type = D3D12_HEAP_TYPE_UPLOAD;

    D3D12_RESOURCE_DESC uploadDesc = {};
    uploadDesc.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
    uploadDesc.Width = uploadSize;
    uploadDesc.Height = 1;
    uploadDesc.DepthOrArraySize = 1;
    uploadDesc.MipLevels = 1;
    uploadDesc.Format = DXGI_FORMAT_UNKNOWN;
    uploadDesc.SampleDesc.Count = 1;
    uploadDesc.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;

    ID3D12Resource *uploadBuffer = nullptr;
    HRESULT hr = device->CreateCommittedResource(
        &uploadProps,
        D3D12_HEAP_FLAG_NONE,
        &uploadDesc,
        D3D12_RESOURCE_STATE_GENERIC_READ,
        nullptr,
        IID_PPV_ARGS(&uploadBuffer));
    if (FAILED(hr))
    {
        stbi_image_free(imageData);
        return false;
    }

    // 复制数据到上传缓冲区
    void *mapped = nullptr;
    D3D12_RANGE range = {0, uploadSize};
    uploadBuffer->Map(0, &range, &mapped);
    for (int y = 0; y < height; y++)
    {
        memcpy((void *)((uintptr_t)mapped + y * uploadPitch),
               imageData + y * width * 4,
               width * 4);
    }
    uploadBuffer->Unmap(0, &range);

    // 设置复制区域
    D3D12_TEXTURE_COPY_LOCATION src = {};
    src.pResource = uploadBuffer;
    src.Type = D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT;
    src.PlacedFootprint.Footprint.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    src.PlacedFootprint.Footprint.Width = width;
    src.PlacedFootprint.Footprint.Height = height;
    src.PlacedFootprint.Footprint.Depth = 1;
    src.PlacedFootprint.Footprint.RowPitch = uploadPitch;

    D3D12_TEXTURE_COPY_LOCATION dst = {};
    dst.pResource = resourceIt->second;
    dst.Type = D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX;
    dst.SubresourceIndex = 0;

    // 创建命令列表并执行复制
    ID3D12CommandAllocator *cmdAlloc;
    ID3D12GraphicsCommandList *cmdList;
    device->CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT, IID_PPV_ARGS(&cmdAlloc));
    device->CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, cmdAlloc, nullptr, IID_PPV_ARGS(&cmdList));

    // 转换资源状态为复制目标
    D3D12_RESOURCE_BARRIER barrier = {};
    barrier.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
    barrier.Flags = D3D12_RESOURCE_BARRIER_FLAG_NONE;
    barrier.Transition.pResource = resourceIt->second;
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_COPY_DEST;
    barrier.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
    cmdList->ResourceBarrier(1, &barrier);

    // 执行复制
    cmdList->CopyTextureRegion(&dst, 0, 0, 0, &src, nullptr);

    // 转换回着色器资源状态
    barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_COPY_DEST;
    barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE;
    cmdList->ResourceBarrier(1, &barrier);

    cmdList->Close();

    // 执行命令并等待完成
    ID3D12Fence *fence = nullptr;
    device->CreateFence(0, D3D12_FENCE_FLAG_NONE, IID_PPV_ARGS(&fence));
    HANDLE event = CreateEvent(nullptr, FALSE, FALSE, nullptr);

    cmdQueue->ExecuteCommandLists(1, (ID3D12CommandList **)&cmdList);
    cmdQueue->Signal(fence, 1);
    fence->SetEventOnCompletion(1, event);
    WaitForSingleObject(event, INFINITE);

    // 清理资源
    stbi_image_free(imageData);
    uploadBuffer->Release();
    cmdList->Release();
    cmdAlloc->Release();
    CloseHandle(event);
    fence->Release();

    return true;
}

ImTextureID TextureManager::GetTextureID(const std::string &key)
{
    auto it = textureMap.find(key);
    if (it != textureMap.end())
    {
        return it->second;
    }
    return (ImTextureID)0;
}

void TextureManager::Release()
{
    // 释放所有纹理资源
    for (auto &pair : resourceMap)
    {
        if (pair.second)
        {
            pair.second->Release();
        }
    }
    resourceMap.clear();
    textureMap.clear();
}
