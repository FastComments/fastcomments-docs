## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| yearNumber | number | Nein |  |
| monthNumber | number | Nein |  |
| dayNumber | number | Nein |  |
| skip | number | Nein |  |

## Antwort

Gibt zurück: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getTenantDailyUsages Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_89f3c2-prod";
const yearNumber: number = 2026;
const monthNumber: number = 1;
const skip: number = 0;
const usages: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, yearNumber, monthNumber, undefined, skip);
[inline-code-end]

---