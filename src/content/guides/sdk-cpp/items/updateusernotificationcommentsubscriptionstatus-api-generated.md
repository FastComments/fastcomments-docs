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
utility::string_t tenantId = U("my-tenant-123");
utility::string_t notificationId = U("notif-456");
utility::string_t optedInOrOut = U("opted_in");
utility::string_t commentId = U("cmt-789");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto defaultResp = std::make_shared<UpdateUserNotificationStatus_200_response>();
api->updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso)
.then([defaultResp](pplx::task<std::shared_ptr<UpdateUserNotificationStatus_200_response>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = defaultResp;
        (void)resp;
    } catch(const std::exception &){
    }
});
[inline-code-end]
