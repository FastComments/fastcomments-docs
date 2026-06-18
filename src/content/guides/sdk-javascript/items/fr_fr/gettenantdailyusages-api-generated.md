---
## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| yearNumber | number | Non |  |
| monthNumber | number | Non |  |
| dayNumber | number | Non |  |
| skip | number | Non |  |

## Response

Retourne: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTenantDailyUsages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7a3c2e';
const dailyUsages: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, 2026, 6, undefined, 0);
[inline-code-end]

---