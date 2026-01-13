req
tenantId
urlId

## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| page | int32_t | いいえ |  |
| direction | SortDirections | いいえ |  |
| sso | string | いいえ |  |
| skip | int32_t | いいえ |  |
| skipChildren | int32_t | いいえ |  |
| limit | int32_t | いいえ |  |
| limitChildren | int32_t | いいえ |  |
| countChildren | bool | いいえ |  |
| fetchPageForCommentId | string | いいえ |  |
| includeConfig | bool | いいえ |  |
| countAll | bool | いいえ |  |
| includei10n | bool | いいえ |  |
| locale | string | いいえ |  |
| modules | string | いいえ |  |
| isCrawler | bool | いいえ |  |
| includeNotificationCount | bool | いいえ |  |
| asTree | bool | いいえ |  |
| maxTreeDepth | int32_t | いいえ |  |
| useFullTranslationIds | bool | いいえ |  |
| parentId | string | いいえ |  |
| searchText | string | いいえ |  |
| hashTags | vector<string | いいえ |  |
| userId | string | いいえ |  |
| customConfigStr | string | いいえ |  |
| afterCommentId | string | いいえ |  |
| beforeCommentId | string | いいえ |  |

## レスポンス

戻り値: [`GetCommentsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsPublic_200_response.h)

## 例

[inline-code-attrs-start title = 'getCommentsPublic の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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