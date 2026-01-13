req
tenantId
urlId

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| page | int32_t | Hayır |  |
| direction | SortDirections | Hayır |  |
| sso | string | Hayır |  |
| skip | int32_t | Hayır |  |
| skipChildren | int32_t | Hayır |  |
| limit | int32_t | Hayır |  |
| limitChildren | int32_t | Hayır |  |
| countChildren | bool | Hayır |  |
| fetchPageForCommentId | string | Hayır |  |
| includeConfig | bool | Hayır |  |
| countAll | bool | Hayır |  |
| includei10n | bool | Hayır |  |
| locale | string | Hayır |  |
| modules | string | Hayır |  |
| isCrawler | bool | Hayır |  |
| includeNotificationCount | bool | Hayır |  |
| asTree | bool | Hayır |  |
| maxTreeDepth | int32_t | Hayır |  |
| useFullTranslationIds | bool | Hayır |  |
| parentId | string | Hayır |  |
| searchText | string | Hayır |  |
| hashTags | vector<string | Hayır |  |
| userId | string | Hayır |  |
| customConfigStr | string | Hayır |  |
| afterCommentId | string | Hayır |  |
| beforeCommentId | string | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsPublic_200_response.h)

## Örnek

[inline-code-attrs-start title = 'getCommentsPublic Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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