## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| notificationId | string | Ja |  |
| newStatus | string | Ja |  |
| sso | string | Nee |  |

## Antwoord

Geeft terug: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatusResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'updateUserNotificationStatus Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t("my-tenant-123");
utility::string_t notificationId = utility::string_t("notif-456");
utility::string_t newStatus = utility::string_t("read");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::string_t("sso-token-xyz"));

api->updateUserNotificationStatus(tenantId, notificationId, newStatus, sso)
.then([](pplx::task<std::shared_ptr<UpdateUserNotificationStatusResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto localCopy = std::make_shared<UpdateUserNotificationStatusResponse>(*resp);
            (void)localCopy;
        }
    } catch (const std::exception& e) {
        (void)e;
    }
});
[inline-code-end]

---