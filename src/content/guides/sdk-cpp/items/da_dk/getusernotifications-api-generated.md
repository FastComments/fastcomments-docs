## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Nej |  |
| pageSize | int32_t | Nej |  |
| afterId | string | Nej |  |
| includeContext | bool | Nej |  |
| afterCreatedAt | int64_t | Nej |  |
| unreadOnly | bool | Nej |  |
| dmOnly | bool | Nej |  |
| noDm | bool | Nej |  |
| includeTranslations | bool | Nej |  |
| includeTenantNotifications | bool | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getUserNotifications Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
api->getUserNotifications(
    tenantId,
    boost::optional<utility::string_t>(U("post-456")),
    boost::optional<int32_t>(50),
    boost::optional<utility::string_t>(U("notif-789")),
    boost::optional<bool>(true),
    boost::optional<int64_t>(1625097600000LL),
    boost::optional<bool>(true),
    boost::optional<bool>(false),
    boost::optional<bool>(false),
    boost::optional<bool>(true),
    boost::optional<bool>(false),
    boost::optional<utility::string_t>(U("user@example.com"))
).then([](pplx::task<std::shared_ptr<GetMyNotificationsResponse>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<GetMyNotificationsResponse>();
        // brug resp, f.eks. gennemgå felter
    } catch(const std::exception &e) {
    }
});
[inline-code-end]

---