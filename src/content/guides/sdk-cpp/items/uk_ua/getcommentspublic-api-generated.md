req
tenantId
urlId

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| page | int32_t | Ні |  |
| direction | SortDirections | Ні |  |
| sso | string | Ні |  |
| skip | int32_t | Ні |  |
| skipChildren | int32_t | Ні |  |
| limit | int32_t | Ні |  |
| limitChildren | int32_t | Ні |  |
| countChildren | bool | Ні |  |
| fetchPageForCommentId | string | Ні |  |
| includeConfig | bool | Ні |  |
| countAll | bool | Ні |  |
| includei10n | bool | Ні |  |
| locale | string | Ні |  |
| modules | string | Ні |  |
| isCrawler | bool | Ні |  |
| includeNotificationCount | bool | Ні |  |
| asTree | bool | Ні |  |
| maxTreeDepth | int32_t | Ні |  |
| useFullTranslationIds | bool | Ні |  |
| parentId | string | Ні |  |
| searchText | string | Ні |  |
| hashTags | vector<string | Ні |  |
| userId | string | Ні |  |
| customConfigStr | string | Ні |  |
| afterCommentId | string | Ні |  |
| beforeCommentId | string | Ні |  |

## Відповідь

Повертає: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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