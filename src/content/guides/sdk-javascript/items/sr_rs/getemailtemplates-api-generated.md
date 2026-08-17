## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Одговор

Враћа: [`GetEmailTemplatesResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplatesResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Primer getEmailTemplates'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";

  // Позив без опционо 'skip'
  const templates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId);

  // Позив са опционо 'skip' параметром
  const pagedTemplates: GetEmailTemplatesResponse1 = await getEmailTemplates(tenantId, 20);
})();
[inline-code-end]

---