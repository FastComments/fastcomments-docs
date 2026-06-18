## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| yearNumber | number | Ne |  |
| monthNumber | number | Ne |  |
| dayNumber | number | Ne |  |
| skip | number | Ne |  |

## Odgovor

Vraća: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## Primer

[inline-code-attrs-start title = 'getTenantDailyUsages Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7a3c2e';
const dailyUsages: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, 2026, 6, undefined, 0);
[inline-code-end]

---