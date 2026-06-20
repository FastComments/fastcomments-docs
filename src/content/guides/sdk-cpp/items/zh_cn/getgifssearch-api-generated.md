---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| search | string | 是 |  |
| locale | string | 否 |  |
| rating | string | 否 |  |
| page | double | 否 |  |

## 响应

返回: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetGifsSearchResponse.h)

## 示例

[inline-code-attrs-start title = 'getGifsSearch 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t search = U("funny cats");
boost::optional<utility::string_t> locale(U("en-US"));
boost::optional<utility::string_t> rating(U("pg"));
boost::optional<double> page(1.0);

api->getGifsSearch(tenantId, search, locale, rating, page)
.then([](pplx::task<std::shared_ptr<GetGifsSearchResponse>> t) {
    try {
        auto resp = t.get();
        auto finalResp = resp ? resp : std::make_shared<GetGifsSearchResponse>();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---