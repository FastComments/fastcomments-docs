Upload and resize an image

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| file | HttpContent | Yes |  |
| sizePreset | SizePreset | No |  |
| urlId | string | No |  |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UploadImageResponse.h)

## Example

[inline-code-attrs-start title = 'uploadImage Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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