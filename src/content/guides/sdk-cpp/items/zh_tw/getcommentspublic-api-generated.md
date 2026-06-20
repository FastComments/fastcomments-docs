req
tenantId
urlId

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| page | int32_t | 否 |  |
| direction | SortDirections | 否 |  |
| sso | string | 否 |  |
| skip | int32_t | 否 |  |
| skipChildren | int32_t | 否 |  |
| limit | int32_t | 否 |  |
| limitChildren | int32_t | 否 |  |
| countChildren | bool | 否 |  |
| fetchPageForCommentId | string | 否 |  |
| includeConfig | bool | 否 |  |
| countAll | bool | 否 |  |
| includei10n | bool | 否 |  |
| locale | string | 否 |  |
| modules | string | 否 |  |
| isCrawler | bool | 否 |  |
| includeNotificationCount | bool | 否 |  |
| asTree | bool | 否 |  |
| maxTreeDepth | int32_t | 否 |  |
| useFullTranslationIds | bool | 否 |  |
| parentId | string | 否 |  |
| searchText | string | 否 |  |
| hashTags | vector<string | 否 |  |
| userId | string | 否 |  |
| customConfigStr | string | 否 |  |
| afterCommentId | string | 否 |  |
| beforeCommentId | string | 否 |  |

## 回應

回傳：[`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## 範例

[inline-code-attrs-start title = 'getCommentsPublic 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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