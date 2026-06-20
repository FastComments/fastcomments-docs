---
req
tenantId
urlId

## Paramètres

| Nom | Type | Obligatoire | Description |
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

Renvoie: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t("my-tenant-123");
auto urlId = utility::string_t("/articles/2026/new-features");
boost::optional<int32_t> page = 1;
boost::optional<SortDirections> direction = SortDirections::DESC;
boost::optional<utility::string_t> sso = utility::string_t("sso-token-abc123");
boost::optional<int32_t> limit = 50;
boost::optional<bool> includeConfig = true;
boost::optional<bool> asTree = true;
boost::optional<int32_t> maxTreeDepth = 3;
std::vector<utility::string_t> tags = { utility::string_t("release"), utility::string_t("cpp") };
boost::optional<std::vector<utility::string_t>> hashTags = tags;
api->getCommentsPublic(tenantId, urlId, page, direction, sso, boost::optional<int32_t>(), boost::optional<int32_t>(), limit, boost::optional<int32_t>(), boost::optional<bool>(), boost::optional<utility::string_t>(), includeConfig, boost::optional<bool>(), boost::optional<bool>(), boost::optional<utility::string_t>(), boost::optional<bool>(), asTree, maxTreeDepth, boost::optional<bool>(), boost::optional<utility::string_t>(), boost::optional<utility::string_t>(), hashTags, boost::optional<utility::string_t>(), boost::optional<utility::string_t>(), boost::optional<utility::string_t>(), boost::optional<utility::string_t>())
.then([](pplx::task<std::shared_ptr<GetCommentsResponseWithPresence_PublicComment_>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<GetCommentsResponseWithPresence_PublicComment_>();
    } catch(...) {}
});
[inline-code-end]

---