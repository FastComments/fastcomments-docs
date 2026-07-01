## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| notificationId | string | Da |  |
| newStatus | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatusResponse.h)

## Primer

[inline-code-attrs-start title = 'updateUserNotificationStatus Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->updateUserNotificationStatus(
    U("my-tenant-123"),
    U("notif-456"),
    U("read"),
    boost::optional<utility::string_t>(U("sso-token-abc"))
).then([](std::shared_ptr<UpdateUserNotificationStatusResponse> resp) {
}).wait();
[inline-code-end]