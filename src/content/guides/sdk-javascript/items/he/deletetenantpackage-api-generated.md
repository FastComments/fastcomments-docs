## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_corp_987';
const packageId: string = 'pkg_pro_2026_01';
type DeleteOptions = { force?: boolean; notify?: boolean };
const options: DeleteOptions = { force: true }; // פרמטרים אופציונליים לדוגמה
const result: FlagCommentPublic200Response = await deleteTenantPackage(tenantId, packageId);
[inline-code-end]

---