## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| yearNumber | number | Nie |  |
| monthNumber | number | Nie |  |
| dayNumber | number | Nie |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getTenantDailyUsages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7a3c2e';
const dailyUsages: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, 2026, 6, undefined, 0);
[inline-code-end]