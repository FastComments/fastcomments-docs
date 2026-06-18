## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |

## Одговор

Враћа: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitions200Response.ts)

## Пример

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_001';
  const options: { includeDrafts?: boolean } = { includeDrafts: true }; // демонстриран опциони параметар
  const templates: GetEmailTemplateDefinitions200Response = await getEmailTemplateDefinitions(tenantId, options);
  console.log(templates);
})();
[inline-code-end]

---