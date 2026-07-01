## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| skip | number | Ні |  |

## Відповідь

Повертає: [`GetEmailTemplatesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'getEmailTemplates Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";

  // Call without optional 'skip'
  const templates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId);

  // Call with optional 'skip' parameter
  const pagedTemplates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId, 20);
})();
[inline-code-end]