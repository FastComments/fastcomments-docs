## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| errorId | string | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של deleteEmailTemplateRenderError'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7a1d2f9b";
const id: string = "email_template_42b1";
const errorId: string = "render_err_2026-04-24_7f3c";
const includeStackTrace: boolean | undefined = undefined; // דוגמה לדגל אופציונלי

const response: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, id, errorId);
// אם אובייקט אפשרויות אופציונלי היה נתמך, זה עשוי להיראות כך:
// await deleteEmailTemplateRenderError(tenantId, id, errorId /*, { includeStackTrace } */);
[inline-code-end]

---