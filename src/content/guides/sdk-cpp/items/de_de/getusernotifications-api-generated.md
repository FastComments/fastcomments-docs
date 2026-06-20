## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Nein |  |
| pageSize | int32_t | Nein |  |
| afterId | string | Nein |  |
| includeContext | bool | Nein |  |
| afterCreatedAt | int64_t | Nein |  |
| unreadOnly | bool | Nein |  |
| dmOnly | bool | Nein |  |
| noDm | bool | Nein |  |
| includeTranslations | bool | Nein |  |
| includeTenantNotifications | bool | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getUserNotifications Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
        // resp verwenden, z. B. Felder inspizieren
    } catch(const std::exception &e) {
    }
});
[inline-code-end]

---