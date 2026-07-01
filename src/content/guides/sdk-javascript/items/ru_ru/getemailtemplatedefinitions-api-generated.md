## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |

## Ответ

Возвращает: [`GetEmailTemplateDefinitionsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Пример getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const emailTemplateDefs: GetEmailTemplateDefinitionsResponse1 = await getEmailTemplateDefinitions(tenantId);
  console.log(emailTemplateDefs);
})();
[inline-code-end]