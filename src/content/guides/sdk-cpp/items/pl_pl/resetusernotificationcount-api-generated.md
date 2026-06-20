## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład resetUserNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->resetUserNotificationCount(tenantId, sso)
    .then([](std::shared_ptr<ResetUserNotificationsResponse> resp){
        if(!resp) resp = std::make_shared<ResetUserNotificationsResponse>();
    })
    .wait();
[inline-code-end]

---