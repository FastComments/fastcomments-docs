---
## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| yearNumber | number | לא |  |
| monthNumber | number | לא |  |
| dayNumber | number | לא |  |
| skip | number | לא |  |

## תגובה

מחזיר: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantDailyUsages200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTenantDailyUsages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7a3c2e';
const dailyUsages: GetTenantDailyUsages200Response = await getTenantDailyUsages(tenantId, 2026, 6, undefined, 0);
[inline-code-end]

---