zahteva
tenantId
urlId

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| page | int32_t | Ne |  |
| direction | SortDirections | Ne |  |
| sso | string | Ne |  |
| skip | int32_t | Ne |  |
| skipChildren | int32_t | Ne |  |
| limit | int32_t | Ne |  |
| limitChildren | int32_t | Ne |  |
| countChildren | bool | Ne |  |
| fetchPageForCommentId | string | Ne |  |
| includeConfig | bool | Ne |  |
| countAll | bool | Ne |  |
| includei10n | bool | Ne |  |
| locale | string | Ne |  |
| modules | string | Ne |  |
| isCrawler | bool | Ne |  |
| includeNotificationCount | bool | Ne |  |
| asTree | bool | Ne |  |
| maxTreeDepth | int32_t | Ne |  |
| useFullTranslationIds | bool | Ne |  |
| parentId | string | Ne |  |
| searchText | string | Ne |  |
| hashTags | vector<string | Ne |  |
| userId | string | Ne |  |
| customConfigStr | string | Ne |  |
| afterCommentId | string | Ne |  |
| beforeCommentId | string | Ne |  |

## Odgovor

Vrne: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Primer

[inline-code-attrs-start title = 'Primer getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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