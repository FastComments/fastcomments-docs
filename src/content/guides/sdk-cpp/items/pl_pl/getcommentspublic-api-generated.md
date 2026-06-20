---
żądanie
tenantId
urlId

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| page | int32_t | Nie |  |
| direction | SortDirections | Nie |  |
| sso | string | Nie |  |
| skip | int32_t | Nie |  |
| skipChildren | int32_t | Nie |  |
| limit | int32_t | Nie |  |
| limitChildren | int32_t | Nie |  |
| countChildren | bool | Nie |  |
| fetchPageForCommentId | string | Nie |  |
| includeConfig | bool | Nie |  |
| countAll | bool | Nie |  |
| includei10n | bool | Nie |  |
| locale | string | Nie |  |
| modules | string | Nie |  |
| isCrawler | bool | Nie |  |
| includeNotificationCount | bool | Nie |  |
| asTree | bool | Nie |  |
| maxTreeDepth | int32_t | Nie |  |
| useFullTranslationIds | bool | Nie |  |
| parentId | string | Nie |  |
| searchText | string | Nie |  |
| hashTags | vector<string | Nie |  |
| userId | string | Nie |  |
| customConfigStr | string | Nie |  |
| afterCommentId | string | Nie |  |
| beforeCommentId | string | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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