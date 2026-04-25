## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| errorId | string | Yes |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteEmailTemplateRenderError'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7a1d2f9b";
const id: string = "email_template_42b1";
const errorId: string = "render_err_2026-04-24_7f3c";
const includeStackTrace: boolean | undefined = undefined; // пример необязательного флага

const response: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, id, errorId);
// Если бы поддерживался необязательный объект options, он мог бы выглядеть так:
// await deleteEmailTemplateRenderError(tenantId, id, errorId /*, { includeStackTrace } */);
[inline-code-end]