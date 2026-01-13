## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di replaceTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9b72f2';
const packageId: string = 'pkg-prod-v2';
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  name: 'Premium Moderation Bundle',
  enabled: true,
  maxModerators: 4,
  // i campi opzionali come "notes" o "trialExpiry" sono intenzionalmente omessi qui
} as ReplaceTenantPackageBody;
const result: FlagCommentPublic200Response = await replaceTenantPackage(
  tenantId,
  packageId,
  replaceTenantPackageBody
);
[inline-code-end]

---