req
tenantId
urlId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | int32_t | Nej |  |
| direction | SortDirections | Nej |  |
| sso | string | Nej |  |
| skip | int32_t | Nej |  |
| skipChildren | int32_t | Nej |  |
| limit | int32_t | Nej |  |
| limitChildren | int32_t | Nej |  |
| countChildren | bool | Nej |  |
| fetchPageForCommentId | string | Nej |  |
| includeConfig | bool | Nej |  |
| countAll | bool | Nej |  |
| includei10n | bool | Nej |  |
| locale | string | Nej |  |
| modules | string | Nej |  |
| isCrawler | bool | Nej |  |
| includeNotificationCount | bool | Nej |  |
| asTree | bool | Nej |  |
| maxTreeDepth | int32_t | Nej |  |
| useFullTranslationIds | bool | Nej |  |
| parentId | string | Nej |  |
| searchText | string | Nej |  |
| hashTags | vector<string | Nej |  |
| userId | string | Nej |  |
| customConfigStr | string | Nej |  |
| afterCommentId | string | Nej |  |
| beforeCommentId | string | Nej |  |

## Svar

Returnerer: [`GetCommentsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsPublic_200_response.h)

## Eksempel

[inline-code-attrs-start title = 'getCommentsPublic Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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