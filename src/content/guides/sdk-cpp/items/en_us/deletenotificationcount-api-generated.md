## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteNotificationCount Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t notifId = U("notification-456");
boost::optional<utility::string_t> actingUser = boost::optional<utility::string_t>(U("moderator@example.com"));
api->deleteNotificationCount(tenantId, notifId)
.then([actingUser](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<FlagCommentPublic_200_response>();
        if (actingUser) std::cout << "Actor: " << actingUser->c_str() << "\n";
        std::cout << "Notification count cleared\n";
    } catch (const std::exception &e) {
        std::cout << "Failed: " << e.what() << "\n";
    }
});
[inline-code-end]