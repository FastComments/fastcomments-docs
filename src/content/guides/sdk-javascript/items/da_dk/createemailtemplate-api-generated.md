## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Ja |  |

## Svar

Returnerer: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplateResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på createEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_prod_7f9b2a";

const customTemplate: CustomEmailTemplate = {
  id: "custtmpl_01",
  name: "MinimalTransactional",
  html: "<div style=\"font-family:Arial,Helvetica,sans-serif\">\{{body}}</div>"
};

const createEmailTemplateBody: CreateEmailTemplateBody = {
  name: "User Welcome - Web",
  subject: "Welcome to Acme — Get Started",
  html: "<p>Hi \{{firstName}}, welcome to Acme!</p>",
  previewText: "Start exploring your new Acme account",
  enabled: true,
  replyTo: "support@acme.com",
  customTemplate // valgfri parameter demonstreret
};

const result: CreateEmailTemplateResponse = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]

---