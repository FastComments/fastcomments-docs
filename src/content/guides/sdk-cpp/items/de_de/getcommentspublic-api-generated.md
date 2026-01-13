req
tenantId
urlId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | int32_t | Nein |  |
| direction | SortDirections | Nein |  |
| sso | string | Nein |  |
| skip | int32_t | Nein |  |
| skipChildren | int32_t | Nein |  |
| limit | int32_t | Nein |  |
| limitChildren | int32_t | Nein |  |
| countChildren | bool | Nein |  |
| fetchPageForCommentId | string | Nein |  |
| includeConfig | bool | Nein |  |
| countAll | bool | Nein |  |
| includei10n | bool | Nein |  |
| locale | string | Nein |  |
| modules | string | Nein |  |
| isCrawler | bool | Nein |  |
| includeNotificationCount | bool | Nein |  |
| asTree | bool | Nein |  |
| maxTreeDepth | int32_t | Nein |  |
| useFullTranslationIds | bool | Nein |  |
| parentId | string | Nein |  |
| searchText | string | Nein |  |
| hashTags | vector<string | Nein |  |
| userId | string | Nein |  |
| customConfigStr | string | Nein |  |
| afterCommentId | string | Nein |  |
| beforeCommentId | string | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsPublic_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'getCommentsPublic Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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