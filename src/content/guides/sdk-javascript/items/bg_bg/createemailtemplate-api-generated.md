## Параметри

| Име | Тип | Изисква се | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Yes |  |

## Отговор

Връща: [`CreateEmailTemplateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplateResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8e7d6c";

const emailTemplate: CreateEmailTemplateBody = {
  name: "Account Activation",
  subject: "Activate Your New Account",
  htmlContent: "<p>Welcome! Please click <a href=\"\{{activationLink}}\">here</a> to activate.</p>",
  // незадължителните полета като textContent, isActive са пропуснати, за да се демонстрират незадължителните параметри
};

const result: CreateEmailTemplateResponse1 = await createEmailTemplate(
  tenantId,
  emailTemplate
);

console.log(result);
[inline-code-end]