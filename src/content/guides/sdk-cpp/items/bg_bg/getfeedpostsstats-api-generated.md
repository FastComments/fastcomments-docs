## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| postIds | vector<string | Да |  |
| sso | string | Не |  |

## Отговор

Връща: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FeedPostsStatsResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за getFeedPostsStats'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
std::vector<utility::string_t> postIds = { U("post-1001"), U("post-1002"), U("post-1003") };
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getFeedPostsStats(tenantId, postIds, sso)
    .then([](pplx::task<std::shared_ptr<FeedPostsStatsResponse>> previous) {
        try {
            auto stats = previous.get();
            if (!stats) stats = std::make_shared<FeedPostsStatsResponse>();
            // обработка на статистиката тук (напр., проверка на полета, обновяване на интерфейса)
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---