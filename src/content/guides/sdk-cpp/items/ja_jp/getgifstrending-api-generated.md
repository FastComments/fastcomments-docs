---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| locale | string | No |  |
| rating | string | No |  |
| page | double | No |  |

## レスポンス

返却値: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetGifsTrendingResponse.h)

## 例

[inline-code-attrs-start title = 'getGifsTrending の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(U("en-US"));
boost::optional<utility::string_t> rating = boost::optional<utility::string_t>(U("pg"));
boost::optional<double> page = boost::optional<double>(2.0);
auto defaultResp = std::make_shared<GetGifsTrendingResponse>();
api->getGifsTrending(tenantId, locale, rating, page)
    .then([defaultResp](std::shared_ptr<GetGifsTrendingResponse> resp) {
        auto r = resp ? resp : defaultResp;
        std::cout << "Received trending GIFs response pointer: " << static_cast<const void*>(r.get()) << std::endl;
    });
[inline-code-end]

---