## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| notificationId | string | Ja |  |
| newStatus | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`UpdateUserNotificationStatus_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatus_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für updateUserNotificationStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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