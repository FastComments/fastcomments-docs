## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| userId | string | Hayır |  |
| urlId | string | Hayır |  |
| fromCommentId | string | Hayır |  |
| viewed | bool | Hayır |  |
| type | string | Hayır |  |

## Yanıt

Döndürür: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationCountResponse.h)

## Örnek

[inline-code-attrs-start title = 'getNotificationCount Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = U("user@example.com");
boost::optional<utility::string_t> urlId = U("article-456");
boost::optional<utility::string_t> fromCommentId = U("cmt-789");
boost::optional<bool> viewed = true;
boost::optional<utility::string_t> type = U("mention");

auto task = api->getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, type)
.then([](pplx::task<std::shared_ptr<GetNotificationCountResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) return std::make_shared<GetNotificationCountResponse>();
        return resp;
    } catch (...) {
        return std::make_shared<GetNotificationCountResponse>();
    }
});
[inline-code-end]