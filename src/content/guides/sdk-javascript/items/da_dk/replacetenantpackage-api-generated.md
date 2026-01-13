## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Ja |  |

## Respons

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på replaceTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9b72f2';
const packageId: string = 'pkg-prod-v2';
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  name: 'Premium Moderation Bundle',
  enabled: true,
  maxModerators: 4,
  // valgfrie felter som "notes" eller "trialExpiry" er bevidst udeladt her
} as ReplaceTenantPackageBody;
const result: FlagCommentPublic200Response = await replaceTenantPackage(
  tenantId,
  packageId,
  replaceTenantPackageBody
);
[inline-code-end]

---