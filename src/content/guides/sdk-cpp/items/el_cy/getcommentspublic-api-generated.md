---
req
tenantId
urlId

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| page | int32_t | Όχι |  |
| direction | SortDirections | Όχι |  |
| sso | string | Όχι |  |
| skip | int32_t | Όχι |  |
| skipChildren | int32_t | Όχι |  |
| limit | int32_t | Όχι |  |
| limitChildren | int32_t | Όχι |  |
| countChildren | bool | Όχι |  |
| fetchPageForCommentId | string | Όχι |  |
| includeConfig | bool | Όχι |  |
| countAll | bool | Όχι |  |
| includei10n | bool | Όχι |  |
| locale | string | Όχι |  |
| modules | string | Όχι |  |
| isCrawler | bool | Όχι |  |
| includeNotificationCount | bool | Όχι |  |
| asTree | bool | Όχι |  |
| maxTreeDepth | int32_t | Όχι |  |
| useFullTranslationIds | bool | Όχι |  |
| parentId | string | Όχι |  |
| searchText | string | Όχι |  |
| hashTags | vector<string | Όχι |  |
| userId | string | Όχι |  |
| customConfigStr | string | Όχι |  |
| afterCommentId | string | Όχι |  |
| beforeCommentId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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