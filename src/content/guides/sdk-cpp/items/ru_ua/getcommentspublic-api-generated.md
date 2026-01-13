запрос
tenantId
urlId

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| page | int32_t | Нет |  |
| direction | SortDirections | Нет |  |
| sso | string | Нет |  |
| skip | int32_t | Нет |  |
| skipChildren | int32_t | Нет |  |
| limit | int32_t | Нет |  |
| limitChildren | int32_t | Нет |  |
| countChildren | bool | Нет |  |
| fetchPageForCommentId | string | Нет |  |
| includeConfig | bool | Нет |  |
| countAll | bool | Нет |  |
| includei10n | bool | Нет |  |
| locale | string | Нет |  |
| modules | string | Нет |  |
| isCrawler | bool | Нет |  |
| includeNotificationCount | bool | Нет |  |
| asTree | bool | Нет |  |
| maxTreeDepth | int32_t | Нет |  |
| useFullTranslationIds | bool | Нет |  |
| parentId | string | Нет |  |
| searchText | string | Нет |  |
| hashTags | vector<string | Нет |  |
| userId | string | Нет |  |
| customConfigStr | string | Нет |  |
| afterCommentId | string | Нет |  |
| beforeCommentId | string | Нет |  |

## Ответ

Возвращает: [`GetCommentsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsPublic_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-2026-01-12");
boost::optional<int32_t> page(2);
boost::optional<SortDirections> direction(SortDirections::DESC);
boost::optional<bool> includeConfig(true);
boost::optional<std::vector<utility::string_t>> hashTags(std::vector<utility::string_t>{ U("feature"), U("bug") });
api->getCommentsPublic(
    tenantId,
    urlId,
    page,
    direction,
    boost::optional<utility::string_t>(),
    boost::optional<int32_t>(),
    boost::optional<int32_t>(),
    boost::optional<int32_t>(50),
    boost::optional<int32_t>(),
    boost::optional<bool>(false),
    boost::optional<utility::string_t>(),
    includeConfig,
    boost::optional<bool>(),
    boost::optional<bool>(),
    boost::optional<utility::string_t>(),
    boost::optional<utility::string_t>(),
    boost::optional<bool>(),
    boost::optional<bool>(),
    boost::optional<bool>(),
    boost::optional<int32_t>(),
    boost::optional<bool>(),
    boost::optional<utility::string_t>(),
    boost::optional<utility::string_t>(),
    hashTags,
    boost::optional<utility::string_t>(U("user-789")),
    boost::optional<utility::string_t>(),
    boost::optional<utility::string_t>()
).then([](pplx::task<std::shared_ptr<GetCommentsPublic_200_response>> t){
    try {
        auto resp = t.get();
        auto fallback = std::make_shared<GetCommentsPublic_200_response>();
        (void)(resp ? resp : fallback);
    } catch (...) {}
});
[inline-code-end]

---