---
req
tenantId
urlId

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| page | int32_t | Ні |  |
| direction | SortDirections | Ні |  |
| sso | string | Ні |  |
| skip | int32_t | Ні |  |
| skipChildren | int32_t | Ні |  |
| limit | int32_t | Ні |  |
| limitChildren | int32_t | Ні |  |
| countChildren | bool | Ні |  |
| fetchPageForCommentId | string | Ні |  |
| includeConfig | bool | Ні |  |
| countAll | bool | Ні |  |
| includei10n | bool | Ні |  |
| locale | string | Ні |  |
| modules | string | Ні |  |
| isCrawler | bool | Ні |  |
| includeNotificationCount | bool | Ні |  |
| asTree | bool | Ні |  |
| maxTreeDepth | int32_t | Ні |  |
| useFullTranslationIds | bool | Ні |  |
| parentId | string | Ні |  |
| searchText | string | Ні |  |
| hashTags | vector<string | Ні |  |
| userId | string | Ні |  |
| customConfigStr | string | Ні |  |
| afterCommentId | string | Ні |  |
| beforeCommentId | string | Ні |  |

## Відповідь

Повертає: [`GetCommentsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsPublic_200_response.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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