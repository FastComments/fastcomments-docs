## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| largeInternalURLSanitized | string | Yes |  |

## Response

Returns: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GifGetLargeResponse.h)

## Example

[inline-code-attrs-start title = 'getGifLarge Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto largeUrl = utility::conversions::to_string_t("https://cdn.example.com/gifs/large/abc123.gif");

api->getGifLarge(tenantId, largeUrl)
   .then([&](std::shared_ptr<GifGetLargeResponse> resp) {
        boost::optional<utility::string_t> maybeUrl;
        if (resp && resp->url) {
            maybeUrl = resp->url;
        }
        if (maybeUrl) {
            std::wcout << *maybeUrl << std::endl;
        }
   });
[inline-code-end]
