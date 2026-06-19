---
## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| domainToUpdate | string | Ναι |  |
| patchDomainConfigParams | PatchDomainConfigParams | Ναι |  |

## Επιστρέφει

Επιστρέφει: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchDomainConfigResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα patchDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8d9f3c4b";
const domainToUpdate: string = "comments.newsroom.example.com";
const patchDomainConfigParams: PatchDomainConfigParams = {
  enabled: true,
  enforceHttps: true, // προαιρετική παράμετρος συμπεριλαμβάνεται
  allowedOrigins: ["https://newsroom.example.com"] // προαιρετική παράμετρος συμπεριλαμβάνεται
};
const result: PatchDomainConfigResponse = await patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
[inline-code-end]

---