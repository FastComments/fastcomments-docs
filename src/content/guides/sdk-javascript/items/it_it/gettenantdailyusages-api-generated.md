## Parametri

| Name | Type | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| yearNumber | number | No |  |
| monthNumber | number | No |  |
| dayNumber | number | No |  |
| skip | number | No |  |

## Risposta

Restituisce: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getTenantDailyUsages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7a3c2e';
const dailyUsages: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, 2026, 6, undefined, 0);
[inline-code-end]

---