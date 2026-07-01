## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| domainToUpdate | string | Da |  |
| patchDomainConfigParams | PatchDomainConfigParams | Da |  |

## Odgovor

Vraća: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchDomainConfigResponse.h)

## Primer

[inline-code-attrs-start title = 'patchDomainConfig Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto domainToUpdate = utility::conversions::to_string_t("example.com");
PatchDomainConfigParams patchDomainConfigParams;
patchDomainConfigParams.enableComments = boost::optional<bool>(true);
patchDomainConfigParams.moderationLevel = boost::optional<int>(2);
patchDomainConfigParams.customHeader = boost::optional<utility::string_t>(utility::conversions::to_string_t("X-Custom-Header"));

api->patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams)
    .then([](std::shared_ptr<PatchDomainConfigResponse> resp) {
        // rukovanje uspehom
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* rukovanje greškom */ }
    });
[inline-code-end]