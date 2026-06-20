req
tenantId
urlId

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| page | int32_t | Не |  |
| direction | SortDirections | Не |  |
| sso | string | Не |  |
| skip | int32_t | Не |  |
| skipChildren | int32_t | Не |  |
| limit | int32_t | Не |  |
| limitChildren | int32_t | Не |  |
| countChildren | bool | Не |  |
| fetchPageForCommentId | string | Не |  |
| includeConfig | bool | Не |  |
| countAll | bool | Не |  |
| includei10n | bool | Не |  |
| locale | string | Не |  |
| modules | string | Не |  |
| isCrawler | bool | Не |  |
| includeNotificationCount | bool | Не |  |
| asTree | bool | Не |  |
| maxTreeDepth | int32_t | Не |  |
| useFullTranslationIds | bool | Не |  |
| parentId | string | Не |  |
| searchText | string | Не |  |
| hashTags | vector<string | Не |  |
| userId | string | Не |  |
| customConfigStr | string | Не |  |
| afterCommentId | string | Не |  |
| beforeCommentId | string | Не |  |

## Отговор

Връща: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Пример

[inline-code-attrs-start title = 'Пример за getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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