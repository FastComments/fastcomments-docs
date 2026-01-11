## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| afterId | string | No |  |
| afterCreatedAt | int64_t | No |  |
| unreadOnly | bool | No |  |
| dmOnly | bool | No |  |
| noDm | bool | No |  |
| sso | string | No |  |

## Response

Returns: [`ResetUserNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotifications_200_response.h)

## Example

[inline-code-attrs-start title = 'resetUserNotifications Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> afterId(utility::conversions::to_string_t("notification-987"));
boost::optional<int64_t> afterCreatedAt(1672531200LL);
boost::optional<bool> unreadOnly(true);
boost::optional<bool> dmOnly(false);
boost::optional<bool> noDm;
boost::optional<utility::string_t> sso(utility::conversions::to_string_t("sso-jwt-abc123"));
auto defaultResp = std::make_shared<ResetUserNotifications_200_response>();
api->resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, dmOnly, noDm, sso)
.then([defaultResp](std::shared_ptr<ResetUserNotifications_200_response> resp){
    auto result = resp ? resp : defaultResp;
    (void)result;
});
[inline-code-end]
