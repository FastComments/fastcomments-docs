---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Όχι |  |
| pageSize | int32_t | Όχι |  |
| afterId | string | Όχι |  |
| includeContext | bool | Όχι |  |
| afterCreatedAt | int64_t | Όχι |  |
| unreadOnly | bool | Όχι |  |
| dmOnly | bool | Όχι |  |
| noDm | bool | Όχι |  |
| includeTranslations | bool | Όχι |  |
| includeTenantNotifications | bool | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
        // χρησιμοποιήστε το resp, π.χ. επιθεωρήστε τα πεδία
    } catch(const std::exception &e) {
    }
});
[inline-code-end]

---