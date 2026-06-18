## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| errorId | string | Так |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteEmailTemplateRenderError'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-72f3b4';
const templateId: string = 'email_template-9c3a1';
let providedErrorId: string | undefined = undefined; // необов'язкове значення, може бути встановлене в іншому місці
const errorId: string = providedErrorId ?? 'render_err-5d2f7';
const result: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, templateId, errorId);
[inline-code-end]