---
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

Gibt zurück: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Beispiel

[inline-code-attrs-start title = 'getCommentsPublic Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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