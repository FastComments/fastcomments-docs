## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-01';
const packageId: string = 'pkg-2026-06-15-001';
const dryRun: boolean | undefined = undefined; // דוגמה לדגל אופציונלי (אינו נדרש על ידי הפונקציה)
const result: FlagCommentPublic200Response = await deleteTenantPackage(tenantId, packageId);
[inline-code-end]

---