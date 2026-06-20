## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nej |  |
| afterCreatedAt | int64_t | Nej |  |
| unreadOnly | bool | Nej |  |
| dmOnly | bool | Nej |  |
| noDm | bool | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## Eksempel

[inline-code-attrs-start title = 'resetUserNotifications Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> afterId = utility::string_t(U("notif-456"));
boost::optional<int64_t> afterCreatedAt = int64_t(1625097600LL);
boost::optional<bool> unreadOnly = true;
boost::optional<bool> dmOnly = false;
boost::optional<bool> noDm = true;
boost::optional<utility::string_t> sso = utility::string_t(U("user@example.com"));
api->resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, dmOnly, noDm, sso)
.then([](pplx::task<std::shared_ptr<ResetUserNotificationsResponse>> t){
    try {
        auto resp = t.get();
        auto result = resp ? resp : std::make_shared<ResetUserNotificationsResponse>();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---