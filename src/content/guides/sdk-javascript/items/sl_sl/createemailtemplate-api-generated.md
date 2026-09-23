## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Yes |  |

## Odgovor

Vrne: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplateResponse.ts)

## Primer

[inline-code-attrs-start title = 'createEmailTemplate Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const templateBody: CreateEmailTemplateBody = {
  name: "Weekly Summary",
  subject: "Your weekly activity report",
  // neobvezno polje
  replyTo: "no-reply@myapp.com",
  htmlContent: "<p>Hello \{{userName}}, here is your summary...</p>"
};

const response: CreateEmailTemplateResponse = await createEmailTemplate(tenantId, templateBody);

console.log(response.template.id);
[inline-code-end]