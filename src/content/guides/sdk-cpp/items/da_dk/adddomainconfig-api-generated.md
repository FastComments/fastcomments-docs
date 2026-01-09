## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| addDomainConfigParams | AddDomainConfigParams | Ja |  |

## Svar

Returnerer: [`AddDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddDomainConfig_200_response.h)

## Eksempel

[inline-code-attrs-start title = 'addDomainConfig Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
AddDomainConfigParams params;
params.domain = U("comments.example.com");
params.allowSubdomains = boost::optional<bool>(true);
params.contactEmail = boost::optional<utility::string_t>(U("admin@example.com"));
api->addDomainConfig(tenantId, params)
.then([](pplx::task<std::shared_ptr<AddDomainConfig_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            auto copy = std::make_shared<AddDomainConfig_200_response>(*resp);
            (void)copy;
        }
    } catch (const std::exception& e) {
        (void)e;
    }
});
[inline-code-end]

---