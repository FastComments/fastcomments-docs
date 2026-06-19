## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| errorId | string | כן |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteEmailTemplateRenderError'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-7f3a2b';
const templateId: string = 'tmpl-9c3e1a2b';
const errorId: string = 'err-2026-06-19-001';
const result: APIEmptyResponse = await deleteEmailTemplateRenderError(tenantId, templateId, errorId);
[inline-code-end]

---