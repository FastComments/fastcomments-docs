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
boost::optional<utility::string_t> auditReason = U("retired-by-admin");
api->deleteUserBadge(tenantId, badgeId).then([=](std::shared_ptr<UpdateUserBadge_200_response> resp){
    auto response = resp ? resp : std::make_shared<UpdateUserBadge_200_response>();
    utility::string_t note = auditReason ? *auditReason : U("");
    (void)note;
    return response;
});
[inline-code-end]
