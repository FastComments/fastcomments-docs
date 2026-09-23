## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Sì |  |

## Risposta

Restituisce: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplateResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio createEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const templateBody: CreateEmailTemplateBody = {
  name: "Weekly Summary",
  subject: "Your weekly activity report",
  // campo opzionale
  replyTo: "no-reply@myapp.com",
  htmlContent: "<p>Hello \{{userName}}, here is your summary...</p>"
};

const response: CreateEmailTemplateResponse = await createEmailTemplate(tenantId, templateBody);

console.log(response.template.id);
[inline-code-end]

---