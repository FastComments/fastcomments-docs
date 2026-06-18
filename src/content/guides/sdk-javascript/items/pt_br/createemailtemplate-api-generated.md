## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Sim |  |

## Resposta

Retorna: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4f2b1c9e";
const createEmailTemplateBody: CreateEmailTemplateBody = {
  name: "New Comment Notification",
  subject: "Someone replied to your discussion",
  fromName: "Community Team",
  fromAddress: "no-reply@community.example.com",
  htmlBody: "<p>\{{comment.author}} replied: \{{comment.text}}</p>",
  plaintextBody: "\{{comment.author}} replied: \{{comment.text}}",
  previewText: "A new reply on a discussion you follow",
  isDefault: false // flag opcional demonstrando o uso de parâmetro opcional
};
const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]

---