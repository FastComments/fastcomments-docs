## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Ответ

Возвращает: [`GetEmailTemplatesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Пример getEmailTemplates'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";

  // Вызов без необязательного 'skip'
  const templates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId);

  // Вызов с необязательным параметром 'skip'
  const pagedTemplates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId, 20);
})();
[inline-code-end]

---