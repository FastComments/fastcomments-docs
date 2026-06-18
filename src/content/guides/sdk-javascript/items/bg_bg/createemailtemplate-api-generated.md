## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Да |  |

## Отговор

Връща: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  isDefault: false // опционален флаг, демонстриращ използването на опционален параметър
};
const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]

---