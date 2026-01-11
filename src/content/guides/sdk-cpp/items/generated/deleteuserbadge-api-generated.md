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
utility::string_t badgeId = U("badge-abc123");
boost::optional<utility::string_t> maybeTenant = tenantId;
api->deleteUserBadge(maybeTenant.value(), badgeId)
.then([](pplx::task<std::shared_ptr<UpdateUserBadge_200_response>> prevTask){
    try {
        auto resp = prevTask.get();
        if (resp) return std::make_shared<UpdateUserBadge_200_response>(*resp);
        return std::make_shared<UpdateUserBadge_200_response>();
    } catch (...) {
        return std::make_shared<UpdateUserBadge_200_response>();
    }
});
[inline-code-end]
