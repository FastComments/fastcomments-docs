## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Da |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer updateEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_67890";

  const updateBody: UpdateEmailTemplateBody = {
    subject: "New Comment Notification",
    // htmlContent je neobvezen in izpuščen
    status: { code: 200, message: "Active" } // APIStatus
  };

  const response: APIEmptyResponse = await updateEmailTemplate(tenantId, templateId, updateBody);
  console.log(response);
}
[inline-code-end]