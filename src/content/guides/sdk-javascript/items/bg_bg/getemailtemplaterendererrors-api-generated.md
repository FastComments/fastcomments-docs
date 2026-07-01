## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| skip | number | Не |  |

## Response

Връща: [`GetEmailTemplateRenderErrorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateRenderErrorsResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getEmailTemplateRenderErrors'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c2f5a8d9-4b3e-4f6a-9e1b-2d5c6f7a8b9c";
const templateId: string = "welcome-email-template";
const skip: number = 20;

const result: GetEmailTemplateRenderErrorsResponse1 = await getEmailTemplateRenderErrors(
  tenantId,
  templateId,
  skip
);
[inline-code-end]