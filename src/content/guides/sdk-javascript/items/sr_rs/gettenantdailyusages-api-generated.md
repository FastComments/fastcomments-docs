## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| yearNumber | number | Не |  |
| monthNumber | number | Не |  |
| dayNumber | number | Не |  |
| skip | number | Не |  |

## Одговор

Враћа: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## Пример

[inline-code-attrs-start title = 'getTenantDailyUsages Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7a3c2e';
const dailyUsages: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, 2026, 6, undefined, 0);
[inline-code-end]

---