---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |

## Отговор

Връща: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateDefinitionsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getEmailTemplateDefinitions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f2c9b1a';
const emailTemplatesResponse: GetEmailTemplateDefinitionsResponse = await getEmailTemplateDefinitions(tenantId);
// Допълнителни параметри (ако се поддържат) могат да бъдат предадени като втори аргумент, напр. getEmailTemplateDefinitions(tenantId /*, { includeDrafts: true } */);
[inline-code-end]

---