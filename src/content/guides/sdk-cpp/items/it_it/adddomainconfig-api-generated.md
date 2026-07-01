## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| addDomainConfigParams | AddDomainConfigParams | Sì |  |

## Risposta

Restituisce: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddDomainConfigResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio addDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
AddDomainConfigParams params;
params.domain = U("example.com");
params.adminEmail = U("admin@example.com");
params.notes = boost::optional<utility::string_t>(U("Primary domain"));
api->addDomainConfig(tenantId, params).then([](pplx::task<std::shared_ptr<AddDomainConfigResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]