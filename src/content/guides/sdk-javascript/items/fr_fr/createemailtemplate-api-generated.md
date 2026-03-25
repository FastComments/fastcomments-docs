## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Oui |  |

## Réponse

Renvoie : [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de createEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7a9f3c2b";
const customConfig: CustomConfigParameters = { smtpHost: "smtp.fastmail.com", smtpPort: 587, useTLS: true };
const createEmailTemplateBody: CreateEmailTemplateBody = {
  name: "Account Notification",
  subject: "Your ACME account was updated",
  fromEmail: "no-reply@acme-corp.com",
  replyTo: "support@acme-corp.com",
  html: "<p>Hi \{{user.firstName}}, your account settings were changed.</p>",
  text: "Hi \{{user.firstName}}, your account settings were changed.",
  isActive: true,
  description: "Used for transactional account update emails",
  customConfig
};
const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]

---