## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| domain | string | Ja |  |

## Antwort

Gibt zurück: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfigResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getDomainConfig Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> domainOpt = U("app.example.com");
if (domainOpt) {
    api->getDomainConfig(tenantId, *domainOpt)
    .then([](pplx::task<std::shared_ptr<GetDomainConfigResponse>> t) {
        try {
            auto resp = t.get();
            auto cfgCopy = std::make_shared<GetDomainConfigResponse>(*resp);
            (void)cfgCopy;
        } catch (const std::exception&) {
        }
    });
}
[inline-code-end]

---