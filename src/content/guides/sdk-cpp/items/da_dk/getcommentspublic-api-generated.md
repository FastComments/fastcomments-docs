req
tenantId
urlId

## Parametre

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

## Respons

Returnerer: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Eksempel

[inline-code-attrs-start title = 'getCommentsPublic Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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