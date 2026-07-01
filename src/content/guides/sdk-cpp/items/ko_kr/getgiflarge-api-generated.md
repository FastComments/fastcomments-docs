## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| largeInternalURLSanitized | string | 예 |  |

## 응답

반환: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GifGetLargeResponse.h)

## 예제

[inline-code-attrs-start title = 'getGifLarge 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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