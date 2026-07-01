## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Odpowiedź

Zwraca: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## Przykład

[inline-code-attrs-start title = 'resetUserNotificationCount Przykład'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto resetTask = api->resetUserNotificationCount(
    U("my-tenant-123"),
    boost::optional<utility::string_t>(U("user@example.com"))
).then([](std::shared_ptr<ResetUserNotificationsResponse> resp){
});
[inline-code-end]