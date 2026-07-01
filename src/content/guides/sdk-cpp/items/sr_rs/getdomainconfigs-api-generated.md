## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |

## Odgovor

Vraća: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigsResponse.h)

## Primer

[inline-code-attrs-start title = 'getDomainConfigs Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> optionalTenant = tenantId;
api->getDomainConfigs(optionalTenant.value())
    .then([](std::shared_ptr<GetDomainConfigsResponse> response) {
        auto domains = response->getDomainList();
        for (const auto& d : domains) {
            std::cout << d << std::endl;
        }
    });
[inline-code-end]