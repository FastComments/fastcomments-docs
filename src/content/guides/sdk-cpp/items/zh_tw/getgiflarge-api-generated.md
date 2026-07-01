## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| largeInternalURLSanitized | string | 是 |  |

## 回應

返回: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GifGetLargeResponse.h)

## 範例

[inline-code-attrs-start title = 'getGifLarge 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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