## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| notificationId | string | Tak |  |
| newStatus | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatusResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład updateUserNotificationStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->updateUserNotificationStatus(
    U("my-tenant-123"),
    U("notif-456"),
    U("read"),
    boost::optional<utility::string_t>(U("sso-token-abc"))
).then([](std::shared_ptr<UpdateUserNotificationStatusResponse> resp) {
}).wait();
[inline-code-end]