## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Yes |  |

## Response

Returns: [`UpdateUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserBadge_200_response.h)

## Example

[inline-code-attrs-start title = 'updateUserBadge Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
auto paramsPtr = std::make_shared<UpdateUserBadgeParams>();
paramsPtr->badgeId = utility::string_t(U("gold-medal"));
paramsPtr->label = boost::optional<utility::string_t>(U("Top Contributor"));
paramsPtr->expiresAt = boost::optional<utility::string_t>(U("2026-12-31T23:59:59Z"));
auto updateTask = api->updateUserBadge(tenantId, id, *paramsPtr)
    .then([](pplx::task<std::shared_ptr<UpdateUserBadge_200_response>> t) {
        try {
            auto resp = t.get();
            return resp;
        } catch (...) {
            return std::shared_ptr<UpdateUserBadge_200_response>();
        }
    });
[inline-code-end]
