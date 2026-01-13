## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| yearNumber | number | Non |  |
| monthNumber | number | Non |  |
| dayNumber | number | Non |  |
| skip | number | Non |  |

## Réponse

Renvoie: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTenantDailyUsages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_89f3c2-prod";
const yearNumber: number = 2026;
const monthNumber: number = 1;
const skip: number = 0;
const usages: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, yearNumber, monthNumber, undefined, skip);
[inline-code-end]

---