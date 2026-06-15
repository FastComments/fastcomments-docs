## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer updateTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2a';
const id: string = 'pkg_pro_2026';
const updateTenantPackageBody: UpdateTenantPackageBody = {
  name: 'Pro Plan',
  monthlyPriceUsd: 49,
  isActive: true,
  features: ['moderation', 'analytics', 'sso'],
  trialDays: 14 // opcioni parametar demonstriran
};
const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]

---