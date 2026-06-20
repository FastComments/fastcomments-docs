## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |

## Одговор

Враћа: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetV1PageLikes.h)

## Пример

[inline-code-attrs-start title = 'getV1PageLikes пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("/articles/2026/new-release");
boost::optional<utility::string_t> includeMeta = boost::optional<utility::string_t>(U("true"));
api->getV1PageLikes(tenantId, urlId).then([](pplx::task<std::shared_ptr<GetV1PageLikes>> task){
    try {
        auto result = task.get();
        auto localCopy = std::make_shared<GetV1PageLikes>(*result);
        (void)localCopy;
    } catch (const std::exception& e) {
        (void)e;
    }
});
[inline-code-end]