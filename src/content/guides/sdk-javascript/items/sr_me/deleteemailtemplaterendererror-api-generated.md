## Параметри

| Име | Тип | Потребно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| errorId | string | Да |  |

## Одговор

Враћа: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b4c2a';
const templateEnvironment: string | undefined = 'production'; // опциони селектор окружења
const id: string = `emailTemplates/${templateEnvironment ?? 'staging'}/welcome_v2`;
const errorId: string = 'err_5a9d2f1c';
const result: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, id, errorId);
console.log(result);
[inline-code-end]

---