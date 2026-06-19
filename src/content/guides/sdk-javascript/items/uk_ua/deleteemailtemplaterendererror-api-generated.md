## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| errorId | string | Так |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Приклад

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-7f3a2b';
const templateId: string = 'tmpl-9c3e1a2b';
const errorId: string = 'err-2026-06-19-001';
const result: APIEmptyResponse = await deleteEmailTemplateRenderError(tenantId, templateId, errorId);
[inline-code-end]

---