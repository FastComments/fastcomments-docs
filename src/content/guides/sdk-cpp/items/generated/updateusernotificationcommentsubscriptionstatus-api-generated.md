## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| notificationId | string | Yes |  |
| optedInOrOut | string | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`UpdateUserNotificationStatus_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatus_200_response.h)

## Example

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t notificationId = utility::conversions::to_string_t("notif-456");
utility::string_t optedInOrOut = utility::conversions::to_string_t("opted_in");
utility::string_t commentId = utility::conversions::to_string_t("comment-789");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::conversions::to_string_t("sso-jwt-token-abc123"));
api->updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso)
.then([](pplx::task<std::shared_ptr<UpdateUserNotificationStatus_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) std::cout << "Subscription updated successfully\n";
        else std::cout << "No response\n";
    } catch (const std::exception& e) {
        std::cout << "Error: " << e.what() << '\n';
    }
});
[inline-code-end]
