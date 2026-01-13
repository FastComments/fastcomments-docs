上传并调整图像大小

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| file | HttpContent | 是 |  |
| sizePreset | SizePreset | 否 |  |
| urlId | string | 否 |  |

## 响应

返回: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UploadImageResponse.h)

## 示例

[inline-code-attrs-start title = 'uploadImage 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto fileBytes = std::vector<unsigned char>{0x89,0x50,0x4E,0x47,0x0D,0x0A,0x1A,0x0A};
auto file = std::make_shared<HttpContent>(fileBytes, U("image/png"), U("avatar.png"));
boost::optional<SizePreset> sizePreset = boost::optional<SizePreset>(SizePreset::MEDIUM);
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("user-avatar-987"));
api->uploadImage(tenantId, file, sizePreset, urlId)
    .then([](pplx::task<std::shared_ptr<UploadImageResponse>> task) {
        try {
            return task.get();
        } catch (...) {
            return std::shared_ptr<UploadImageResponse>();
        }
    });
[inline-code-end]

---