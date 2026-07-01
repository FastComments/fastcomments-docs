## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| largeInternalURLSanitized | string | Da |  |

## Odgovor

Vrne: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GifGetLargeResponse.h)

## Primer

[inline-code-attrs-start title = 'getGifLarge Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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