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
auto fileContent = std::make_shared<HttpContent>(U("avatar.png"), U("image/png"));
boost::optional<SizePreset> sizePreset = boost::optional<SizePreset>(SizePreset::MEDIUM);
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("profile-avatar"));
api->uploadImage(tenantId, *fileContent, sizePreset, urlId)
.then([](std::shared_ptr<UploadImageResponse> resp){
    if (resp) std::cout << "Image uploaded successfully" << std::endl;
    else std::cerr << "Image upload returned no response" << std::endl;
});
[inline-code-end]
