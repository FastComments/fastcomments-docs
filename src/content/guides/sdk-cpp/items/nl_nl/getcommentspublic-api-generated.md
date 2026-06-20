---

req
tenantId
urlId

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | int32_t | Nee |  |
| direction | SortDirections | Nee |  |
| sso | string | Nee |  |
| skip | int32_t | Nee |  |
| skipChildren | int32_t | Nee |  |
| limit | int32_t | Nee |  |
| limitChildren | int32_t | Nee |  |
| countChildren | bool | Nee |  |
| fetchPageForCommentId | string | Nee |  |
| includeConfig | bool | Nee |  |
| countAll | bool | Nee |  |
| includei10n | bool | Nee |  |
| locale | string | Nee |  |
| modules | string | Nee |  |
| isCrawler | bool | Nee |  |
| includeNotificationCount | bool | Nee |  |
| asTree | bool | Nee |  |
| maxTreeDepth | int32_t | Nee |  |
| useFullTranslationIds | bool | Nee |  |
| parentId | string | Nee |  |
| searchText | string | Nee |  |
| hashTags | vector<string | Nee |  |
| userId | string | Nee |  |
| customConfigStr | string | Nee |  |
| afterCommentId | string | Nee |  |
| beforeCommentId | string | Nee |  |

## Antwoord

Geeft terug: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsPublic Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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