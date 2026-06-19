## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| domainToUpdate | string | Oui |  |
| patchDomainConfigParams | PatchDomainConfigParams | Oui |  |

## Réponse

Renvoie: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchDomainConfigResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de patchDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8d9f3c4b";
const domainToUpdate: string = "comments.newsroom.example.com";
const patchDomainConfigParams: PatchDomainConfigParams = {
  enabled: true,
  enforceHttps: true, // paramètre optionnel inclus
  allowedOrigins: ["https://newsroom.example.com"] // paramètre optionnel inclus
};
const result: PatchDomainConfigResponse = await patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
[inline-code-end]

---