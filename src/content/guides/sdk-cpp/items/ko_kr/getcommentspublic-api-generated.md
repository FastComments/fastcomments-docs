req
tenantId
urlId

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| page | int32_t | 아니오 |  |
| direction | SortDirections | 아니오 |  |
| sso | string | 아니오 |  |
| skip | int32_t | 아니오 |  |
| skipChildren | int32_t | 아니오 |  |
| limit | int32_t | 아니오 |  |
| limitChildren | int32_t | 아니오 |  |
| countChildren | bool | 아니오 |  |
| fetchPageForCommentId | string | 아니오 |  |
| includeConfig | bool | 아니오 |  |
| countAll | bool | 아니오 |  |
| includei10n | bool | 아니오 |  |
| locale | string | 아니오 |  |
| modules | string | 아니오 |  |
| isCrawler | bool | 아니오 |  |
| includeNotificationCount | bool | 아니오 |  |
| asTree | bool | 아니오 |  |
| maxTreeDepth | int32_t | 아니오 |  |
| useFullTranslationIds | bool | 아니오 |  |
| parentId | string | 아니오 |  |
| searchText | string | 아니오 |  |
| hashTags | vector<string | 아니오 |  |
| userId | string | 아니오 |  |
| customConfigStr | string | 아니오 |  |
| afterCommentId | string | 아니오 |  |
| beforeCommentId | string | 아니오 |  |

## 응답

반환: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## 예제

[inline-code-attrs-start title = 'getCommentsPublic 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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