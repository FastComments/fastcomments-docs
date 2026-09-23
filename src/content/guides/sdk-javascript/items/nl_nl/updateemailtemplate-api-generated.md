## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Ja |  |

## Respons

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'updateEmailTemplate Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_67890";

  const updateBody: UpdateEmailTemplateBody = {
    subject: "New Comment Notification",
    // htmlContent is optional and omitted
    // htmlContent is optioneel en weggelaten
    status: { code: 200, message: "Active" } // APIStatus
  };

  const response: APIEmptyResponse = await updateEmailTemplate(tenantId, templateId, updateBody);
  console.log(response);
}
[inline-code-end]