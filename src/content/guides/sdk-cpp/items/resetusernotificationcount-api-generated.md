## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`ResetUserNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotifications_200_response.h)

## Example

[inline-code-attrs-start title = 'resetUserNotificationCount Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> sso(utility::conversions::to_string_t("user@example.com"));
api->resetUserNotificationCount(tenantId, sso)
.then([](std::shared_ptr<ResetUserNotifications_200_response> resp){
    if(!resp) return;
    auto localCopy = std::make_shared<ResetUserNotifications_200_response>(*resp);
})
.wait();
[inline-code-end]
