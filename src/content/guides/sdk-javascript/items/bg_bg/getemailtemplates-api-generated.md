## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| skip | number | Не |  |

## Отговор

Връща: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplates200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getEmailTemplates'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = 'tenant_5f3a9c2b';
  const templates: GetEmailTemplates200Response = await getEmailTemplates(tenantId);
  const skip: number = 20;
  const pagedTemplates: GetEmailTemplates200Response = await getEmailTemplates(tenantId, skip);
  console.log(templates, pagedTemplates);
}
main();
[inline-code-end]

---