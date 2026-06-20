---
为特定评论启用或禁用通知。

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| notificationId | string | 是 |  |
| optedInOrOut | string | 是 |  |
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationCommentSubscriptionStatusResponse.h)

## 示例

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t notificationId = U("notif-456");
utility::string_t optedInOrOut = U("opted_in");
utility::string_t commentId = U("cmt-789");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-jwt-abc123"));
api->updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso)
.then([](pplx::task<std::shared_ptr<UpdateUserNotificationCommentSubscriptionStatusResponse>> t) {
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<UpdateUserNotificationCommentSubscriptionStatusResponse>();
        std::cout << "Subscription update completed" << std::endl;
    } catch(const std::exception& e) {
        std::cout << "Error updating subscription: " << e.what() << std::endl;
    }
});
[inline-code-end]

---