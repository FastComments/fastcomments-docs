## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| domain | string | Da |  |

## Odgovor

Vrne: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("myblog.example.com");

api->getDomainConfig(tenantId, domain)
    .then([](std::shared_ptr<GetDomainConfigResponse> response) {
        if (!response) return;
        boost::optional<bool> moderationEnabled = response->moderationEnabled;
        boost::optional<std::string> theme = response->theme;
        if (moderationEnabled && *moderationEnabled) {
            // obravnava omogočeno moderiranje
        }
        if (theme) {
            // uporabi vrednost teme
        }
    })
    .wait();
[inline-code-end]