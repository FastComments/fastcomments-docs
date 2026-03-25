## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenant200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f4b2c1a";
const idOverride: string | undefined = undefined; // החלפה אופציונלית, אם זמינה
const id: string = idOverride ?? "site_3e7a6b2f";
const response: GetTenant200Response = await getTenant(tenantId, id);
console.log(response);
[inline-code-end]

---