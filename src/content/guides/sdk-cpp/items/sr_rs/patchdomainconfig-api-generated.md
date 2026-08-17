## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| domainToUpdate | string | Да |  |
| patchDomainConfigParams | PatchDomainConfigParams | Да |  |

## Одговор

Враћа: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PatchDomainConfigResponse.h)

## Пример

[inline-code-attrs-start title = 'patchDomainConfig Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto domainToUpdate = utility::conversions::to_string_t("example.com");
PatchDomainConfigParams patchDomainConfigParams;
patchDomainConfigParams.enableComments = boost::optional<bool>(true);
patchDomainConfigParams.moderationLevel = boost::optional<int>(2);
patchDomainConfigParams.customHeader = boost::optional<utility::string_t>(utility::conversions::to_string_t("X-Custom-Header"));

api->patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams)
    .then([](std::shared_ptr<PatchDomainConfigResponse> resp) {
        // обрада успеха
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* обрада грешке */ }
    });
[inline-code-end]