## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| page | int32_t | Non |  |
| limit | int32_t | Non |  |
| skip | int32_t | Non |  |
| asTree | bool | Non |  |
| skipChildren | int32_t | Non |  |
| limitChildren | int32_t | Non |  |
| maxTreeDepth | int32_t | Non |  |
| urlId | string | Non |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |
| contextUserId | string | Non |  |
| hashTag | string | Non |  |
| parentId | string | Non |  |
| direction | SortDirections | Non |  |

## Réponse

Renvoie: [`GetComments_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetComments_200_response.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getComments'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<int32_t> page(1);
boost::optional<int32_t> limit(50);
boost::optional<int32_t> skip(0);
boost::optional<bool> asTree(true);
boost::optional<int32_t> skipChildren(0);
boost::optional<int32_t> limitChildren(10);
boost::optional<int32_t> maxTreeDepth(3);
boost::optional<utility::string_t> urlId(U("/articles/2025/fast-api"));
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<utility::string_t> anonUserId(U("anon-abc-123"));
boost::optional<utility::string_t> contextUserId(U("context-user-789"));
boost::optional<utility::string_t> hashTag(U("release"));
boost::optional<utility::string_t> parentId(U("parent-comment-456"));
boost::optional<SortDirections> direction(SortDirections::DESCENDING);

api->getComments(tenantId, page, limit, skip, asTree, skipChildren, limitChildren, maxTreeDepth, urlId, userId, anonUserId, contextUserId, hashTag, parentId, direction)
.then([](pplx::task<std::shared_ptr<GetComments_200_response>> task){
    try {
        auto response = task.get();
        if (!response) response = std::make_shared<GetComments_200_response>();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---