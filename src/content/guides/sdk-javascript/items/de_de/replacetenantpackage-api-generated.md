## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Ja |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'replaceTenantPackage Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9b72f2';
const packageId: string = 'pkg-prod-v2';
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  name: 'Premium Moderation Bundle',
  enabled: true,
  maxModerators: 4,
  // optionale Felder wie "notes" oder "trialExpiry" sind hier absichtlich weggelassen
} as ReplaceTenantPackageBody;
const result: FlagCommentPublic200Response = await replaceTenantPackage(
  tenantId,
  packageId,
  replaceTenantPackageBody
);
[inline-code-end]

---