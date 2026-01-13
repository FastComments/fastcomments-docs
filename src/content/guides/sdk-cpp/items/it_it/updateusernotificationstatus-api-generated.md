## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| notificationId | string | Sì |  |
| newStatus | string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`UpdateUserNotificationStatus_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatus_200_response.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateUserNotificationStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t notificationId = U("notification-789");
utility::string_t newStatus = U("read");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-abc123"));
api->updateUserNotificationStatus(tenantId, notificationId, newStatus, sso)
.then([](pplx::task<std::shared_ptr<UpdateUserNotificationStatus_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto result = std::make_shared<UpdateUserNotificationStatus_200_response>(*resp);
        }
    } catch (...) {
    }
});
[inline-code-end]

---