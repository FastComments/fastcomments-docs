## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Yes |  |

## Відповідь

Повертає: [`CreateEmailTemplateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplateResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'createEmailTemplate Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8e7d6c";

const emailTemplate: CreateEmailTemplateBody = {
  name: "Account Activation",
  subject: "Activate Your New Account",
  htmlContent: "<p>Welcome! Please click <a href=\"\{{activationLink}}\">here</a> to activate.</p>",
  // опціональні поля, такі як textContent, isActive, пропущені, щоб продемонструвати необов’язкові параметри
};

const result: CreateEmailTemplateResponse1 = await createEmailTemplate(
  tenantId,
  emailTemplate
);

console.log(result);
[inline-code-end]