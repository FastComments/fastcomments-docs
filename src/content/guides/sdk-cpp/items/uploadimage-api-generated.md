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
auto file = std::make_shared<HttpContent>(U("avatar.png"), std::vector<unsigned char>{137,80,78,71}, U("image/png"));
boost::optional<SizePreset> sizePreset = boost::optional<SizePreset>(SizePreset::Medium);
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("avatar-2026"));
std::shared_ptr<UploadImageResponse> uploadedResp;
api->uploadImage(tenantId, file, sizePreset, urlId).then([&uploadedResp](pplx::task<std::shared_ptr<UploadImageResponse>> t) {
    try {
        uploadedResp = t.get();
    } catch (...) {
        uploadedResp.reset();
    }
});
[inline-code-end]
