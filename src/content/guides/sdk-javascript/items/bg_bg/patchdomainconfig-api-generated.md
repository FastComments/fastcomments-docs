## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| domainToUpdate | string | Да |  |
| patchDomainConfigParams | PatchDomainConfigParams | Да |  |

## Отговор

Връща: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchDomainConfigResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за patchDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8d9f3c4b";
const domainToUpdate: string = "comments.newsroom.example.com";
const patchDomainConfigParams: PatchDomainConfigParams = {
  enabled: true,
  enforceHttps: true, // включен незадължителен параметър
  allowedOrigins: ["https://newsroom.example.com"] // включен незадължителен параметър
};
const result: PatchDomainConfigResponse = await patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
[inline-code-end]

---