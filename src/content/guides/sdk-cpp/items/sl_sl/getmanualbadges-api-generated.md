## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantManualBadgesResponse.h)

## Primer

[inline-code-attrs-start title = 'getManualBadges Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::make_optional(U("user@example.com"));

api->getManualBadges(tenantId, sso)
    .then([](pplx::task<std::shared_ptr<GetTenantManualBadgesResponse>> t) {
        try {
            auto response = t.get();
            // obdelaj odgovor, npr., response->badgeList
        } catch (const std::exception& ex) {
            // obravnavaj napako
        }
    });
[inline-code-end]