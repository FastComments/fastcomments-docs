## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| sso | string | Non |  |

## Response

Renvoie : [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserNotificationCountResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getUserNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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