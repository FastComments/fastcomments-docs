Włączanie lub wyłączanie powiadomień dla konkretnego komentarza.

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| notificationId | string | Tak |  |
| optedInOrOut | string | Tak |  |
| commentId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationCommentSubscriptionStatusResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład updateUserNotificationCommentSubscriptionStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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