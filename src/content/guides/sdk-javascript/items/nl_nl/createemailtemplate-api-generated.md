## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Ja |  |

## Respons

Retourneert: [`CreateEmailTemplateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplateResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld createEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8e7d6c";

const emailTemplate: CreateEmailTemplateBody = {
  name: "Account Activation",
  subject: "Activate Your New Account",
  htmlContent: "<p>Welcome! Please click <a href=\"\\{{activationLink}}\">here</a> to activate.</p>",
  // optionele velden zoals textContent, isActive zijn weggelaten om optionele parameters te demonstreren
};

const result: CreateEmailTemplateResponse1 = await createEmailTemplate(
  tenantId,
  emailTemplate
);

console.log(result);
[inline-code-end]

---