## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| domainToUpdate | string | Evet |  |
| patchDomainConfigParams | PatchDomainConfigParams | Evet |  |

## Yanıt

Döndürür: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchDomainConfigResponse.ts)

## Örnek

[inline-code-attrs-start title = 'patchDomainConfig Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8d9f3c4b";
const domainToUpdate: string = "comments.newsroom.example.com";
const patchDomainConfigParams: PatchDomainConfigParams = {
  enabled: true,
  enforceHttps: true, // isteğe bağlı parametre dahil edildi
  allowedOrigins: ["https://newsroom.example.com"] // isteğe bağlı parametre dahil edildi
};
const result: PatchDomainConfigResponse = await patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
[inline-code-end]

---