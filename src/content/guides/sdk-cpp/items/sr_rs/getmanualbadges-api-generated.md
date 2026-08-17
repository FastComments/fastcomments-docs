## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Одговор

Враћа: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetTenantManualBadgesResponse.h)

## Пример

[inline-code-attrs-start title = 'Primer getManualBadges'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::make_optional(U("user@example.com"));

api->getManualBadges(tenantId, sso)
    .then([](pplx::task<std::shared_ptr<GetTenantManualBadgesResponse>> t) {
        try {
            auto response = t.get();
            // обради одговор, нпр., response->badgeList
        } catch (const std::exception& ex) {
            // обради грешку
        }
    });
[inline-code-end]