## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| yearNumber | number | Nee |  |
| monthNumber | number | Nee |  |
| dayNumber | number | Nee |  |
| skip | number | Nee |  |

## Respons

Retourneert: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTenantDailyUsages Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f4a3b2c-1d6e-4f9a-b9d8-123456789abc';
const yearNumber: number = 2026;
const monthNumber: number = 3;
const dayNumber: number = 24;
const skip: number = 0;

const result: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, yearNumber, monthNumber, dayNumber, skip);
[inline-code-end]

---