## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| yearNumber | number | Nej |  |
| monthNumber | number | Nej |  |
| dayNumber | number | Nej |  |
| skip | number | Nej |  |

## Respons

Returnerer: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getTenantDailyUsages Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_89f3c2-prod";
const yearNumber: number = 2026;
const monthNumber: number = 1;
const skip: number = 0;
const usages: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, yearNumber, monthNumber, undefined, skip);
[inline-code-end]

---