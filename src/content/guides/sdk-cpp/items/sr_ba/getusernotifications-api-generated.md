## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Ne |  |
| pageSize | int32_t | Ne |  |
| afterId | string | Ne |  |
| includeContext | bool | Ne |  |
| afterCreatedAt | int64_t | Ne |  |
| unreadOnly | bool | Ne |  |
| dmOnly | bool | Ne |  |
| noDm | bool | Ne |  |
| includeTranslations | bool | Ne |  |
| includeTenantNotifications | bool | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## Primjer

[inline-code-attrs-start title = 'Primjer getUserNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
        // koristi resp, npr. provjeri polja
    } catch(const std::exception &e) {
    }
});
[inline-code-end]

---