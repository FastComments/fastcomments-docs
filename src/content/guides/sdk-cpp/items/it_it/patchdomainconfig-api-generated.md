## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| patchDomainConfigParams | PatchDomainConfigParams | Yes |  |

## Risposta

Restituisce: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchDomainConfigResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio patchDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto domainToUpdate = utility::conversions::to_string_t("example.com");
PatchDomainConfigParams patchDomainConfigParams;
patchDomainConfigParams.enableComments = boost::optional<bool>(true);
patchDomainConfigParams.moderationLevel = boost::optional<int>(2);
patchDomainConfigParams.customHeader = boost::optional<utility::string_t>(utility::conversions::to_string_t("X-Custom-Header"));

api->patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams)
    .then([](std::shared_ptr<PatchDomainConfigResponse> resp) {
        // gestione del successo
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* gestione dell'errore */ }
    });
[inline-code-end]