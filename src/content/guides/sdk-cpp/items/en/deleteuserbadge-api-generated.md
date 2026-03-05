## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`UpdateUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserBadge_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteUserBadge Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t badgeId = U("user-badge-789");
boost::optional<utility::string_t> actingUser = boost::optional<utility::string_t>(U("admin@example.com"));
api->deleteUserBadge(tenantId, badgeId)
.then([actingUser](pplx::task<std::shared_ptr<UpdateUserBadge_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            auto result = resp;
        } else {
            auto result = std::make_shared<UpdateUserBadge_200_response>();
        }
    } catch (const std::exception&) {
        auto errFallback = std::make_shared<UpdateUserBadge_200_response>();
    }
});
[inline-code-end]
