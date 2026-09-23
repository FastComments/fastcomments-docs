## Параметри

| Name | Type | Required | Опис |
|------|------|----------|------|
| tenantId | string | Так |  |
| id | string | Так |  |
| skip | number | Ні |  |

## Відповідь

Повертає: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateRenderErrorsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getEmailTemplateRenderErrors'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_9876";

  const resultWithSkip: GetEmailTemplateRenderErrorsResponse = await getEmailTemplateRenderErrors(tenantId, templateId, 10);
  const resultWithoutSkip: GetEmailTemplateRenderErrorsResponse = await getEmailTemplateRenderErrors(tenantId, templateId);

  console.log(resultWithSkip, resultWithoutSkip);
}
demo();
[inline-code-end]