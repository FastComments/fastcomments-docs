## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| afterId | string | Non |  |
| afterCreatedAt | int64_t | Non |  |
| unreadOnly | bool | Non |  |
| dmOnly | bool | Non |  |
| noDm | bool | Non |  |
| sso | string | Non |  |

## Réponse

Retourne : [`ResetUserNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotifications_200_response.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de resetUserNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> afterId = boost::optional<utility::string_t>(U("notif-987654321"));
boost::optional<int64_t> afterCreatedAt = boost::optional<int64_t>(1625097600000LL);
boost::optional<bool> unreadOnly = boost::optional<bool>(true);
boost::optional<bool> dmOnly = boost::optional<bool>(false);
boost::optional<bool> noDm;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, dmOnly, noDm, sso)
    .then([](pplx::task<std::shared_ptr<ResetUserNotifications_200_response>> t)
    {
        try
        {
            auto resp = t.get();
            auto respCopy = std::make_shared<ResetUserNotifications_200_response>(*resp);
            return respCopy;
        }
        catch (...)
        {
            return std::shared_ptr<ResetUserNotifications_200_response>();
        }
    });
[inline-code-end]

---