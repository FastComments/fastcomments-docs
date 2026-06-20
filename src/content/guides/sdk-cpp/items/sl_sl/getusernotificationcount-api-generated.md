## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserNotificationCountResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getUserNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto fallback = std::make_shared<GetUserNotificationCountResponse>();
api->getUserNotificationCount(tenantId, sso)
.then([fallback](std::shared_ptr<GetUserNotificationCountResponse> resp) {
    auto result = resp ? resp : fallback;
    std::cout << "Received user notification count response (ptr=" << (result.get() != nullptr) << ")\n";
})
.wait();
[inline-code-end]

---