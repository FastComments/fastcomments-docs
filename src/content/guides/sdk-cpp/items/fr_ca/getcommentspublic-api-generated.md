req
tenantId
urlId

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| page | int32_t | Non |  |
| direction | SortDirections | Non |  |
| sso | string | Non |  |
| skip | int32_t | Non |  |
| skipChildren | int32_t | Non |  |
| limit | int32_t | Non |  |
| limitChildren | int32_t | Non |  |
| countChildren | bool | Non |  |
| fetchPageForCommentId | string | Non |  |
| includeConfig | bool | Non |  |
| countAll | bool | Non |  |
| includei10n | bool | Non |  |
| locale | string | Non |  |
| modules | string | Non |  |
| isCrawler | bool | Non |  |
| includeNotificationCount | bool | Non |  |
| asTree | bool | Non |  |
| maxTreeDepth | int32_t | Non |  |
| useFullTranslationIds | bool | Non |  |
| parentId | string | Non |  |
| searchText | string | Non |  |
| hashTags | vector<string | Non |  |
| userId | string | Non |  |
| customConfigStr | string | Non |  |
| afterCommentId | string | Non |  |
| beforeCommentId | string | Non |  |

## Réponse

Renvoie: [`GetCommentsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsPublic_200_response.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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