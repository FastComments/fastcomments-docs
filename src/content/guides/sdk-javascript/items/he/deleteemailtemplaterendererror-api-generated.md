---
## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| errorId | string | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteEmailTemplateRenderError'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-72f3b4';
const templateId: string = 'email_template-9c3a1';
let providedErrorId: string | undefined = undefined; // ערך אופציונלי, עשוי להיות מוגדר במקום אחר
const errorId: string = providedErrorId ?? 'render_err-5d2f7';
const result: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, templateId, errorId);
[inline-code-end]

---