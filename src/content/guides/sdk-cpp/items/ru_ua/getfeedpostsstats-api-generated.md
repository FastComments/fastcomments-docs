## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | vector<string | Yes |  |
| sso | string | No |  |

## Ответ

Возвращает: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FeedPostsStatsResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getFeedPostsStats'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
std::vector<utility::string_t> postIds = {
    utility::conversions::to_string_t("post-001"),
    utility::conversions::to_string_t("post-002")
};
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("user@example.com");

api->getFeedPostsStats(tenantId, postIds, sso)
    .then([](std::shared_ptr<FeedPostsStatsResponse> response) {
        (void)response;
    });
[inline-code-end]