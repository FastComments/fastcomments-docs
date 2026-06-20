## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| domainToUpdate | string | Sì |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Sì |  |

## Risposta

Restituisce: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutDomainConfigResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di putDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("example.com");
boost::optional<utility::string_t> contactEmail = U("admin@example.com");
boost::optional<bool> enforceHttps = true;
UpdateDomainConfigParams updateParams;
updateParams.contactEmail = contactEmail;
updateParams.enforceHttps = enforceHttps;
api->putDomainConfig(tenantId, domainToUpdate, updateParams)
.then([](pplx::task<std::shared_ptr<PutDomainConfigResponse>> t){
    try {
        auto resp = t.get();
        return resp ? resp : std::make_shared<PutDomainConfigResponse>();
    } catch(...) {
        return std::make_shared<PutDomainConfigResponse>();
    }
});
[inline-code-end]

---