## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| meta | string | לא |  |
| skip | number | לא |  |

## תגובה

מחזיר: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTenants'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-72b';
const meta: string = 'include=domains,billing';
const skip: number = 20;
const result: GetTenantsResponse = await getTenants(tenantId, meta, skip);
[inline-code-end]

---