req
tenantId
urlId

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| page | int32_t | לא |  |
| direction | SortDirections | לא |  |
| sso | string | לא |  |
| skip | int32_t | לא |  |
| skipChildren | int32_t | לא |  |
| limit | int32_t | לא |  |
| limitChildren | int32_t | לא |  |
| countChildren | bool | לא |  |
| fetchPageForCommentId | string | לא |  |
| includeConfig | bool | לא |  |
| countAll | bool | לא |  |
| includei10n | bool | לא |  |
| locale | string | לא |  |
| modules | string | לא |  |
| isCrawler | bool | לא |  |
| includeNotificationCount | bool | לא |  |
| asTree | bool | לא |  |
| maxTreeDepth | int32_t | לא |  |
| useFullTranslationIds | bool | לא |  |
| parentId | string | לא |  |
| searchText | string | לא |  |
| hashTags | vector<string | לא |  |
| userId | string | לא |  |
| customConfigStr | string | לא |  |
| afterCommentId | string | לא |  |
| beforeCommentId | string | לא |  |

## תגובה

מחזיר: [`GetCommentsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsPublic_200_response.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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