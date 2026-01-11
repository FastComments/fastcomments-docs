## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| notificationId | string | Yes |  |
| newStatus | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`UpdateUserNotificationStatus_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatus_200_response.h)

## Example

[inline-code-attrs-start title = 'updateUserNotificationStatus Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t("my-tenant-123");
utility::string_t notificationId = utility::string_t("notif-98765");
utility::string_t newStatus = utility::string_t("read");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::string_t("user@example.com"));

api->updateUserNotificationStatus(tenantId, notificationId, newStatus, sso)
.then([](pplx::task<std::shared_ptr<UpdateUserNotificationStatus_200_response>> task) {
    try {
        auto resp = task.get();
        auto safeResp = resp ? resp : std::make_shared<UpdateUserNotificationStatus_200_response>();
        (void)safeResp;
    } catch (...) {
    }
}).wait();
[inline-code-end]
