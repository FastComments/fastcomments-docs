## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Antwort

Rückgabe: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEmailTemplateResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getEmailTemplate Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTemplate() {
  const tenantId: string = "acme-corp-123";
  const templateId: string = "welcome-email-456";
  const response: GetEmailTemplateResponse = await getEmailTemplate(tenantId, templateId);
  const status: APIStatus | undefined = response.status;
  const customTemplate: CustomEmailTemplate | undefined = response.template;
}
[inline-code-end]

---