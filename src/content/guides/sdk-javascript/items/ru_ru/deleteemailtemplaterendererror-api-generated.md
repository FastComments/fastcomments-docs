## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| errorId | string | Да |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeRenderError() {
  const tenantId: string = "acme-corp-tenant";
  const templateId: string = "welcome-email-template";
  const errorId: string = "render-err-20230915";

  const result: APIEmptyResponse = await deleteEmailTemplateRenderError(tenantId, templateId, errorId);
}
[inline-code-end]

---