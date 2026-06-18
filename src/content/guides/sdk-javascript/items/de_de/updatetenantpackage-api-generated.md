## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Ja |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'updateTenantPackage Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2a';
const id: string = 'pkg_pro_2026';
const updateTenantPackageBody: UpdateTenantPackageBody = {
  name: 'Pro Plan',
  monthlyPriceUsd: 49,
  isActive: true,
  features: ['moderation', 'analytics', 'sso'],
  trialDays: 14 // optionaler Parameter zur Demonstration
};
const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]

---