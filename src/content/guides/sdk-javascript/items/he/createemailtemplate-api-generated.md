## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createEmailTemplateBody | CreateEmailTemplateBody | כן |  |

## Response

מחזיר: [`CreateEmailTemplateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplateResponse1.ts)

## Example

[inline-code-attrs-start title = 'דוגמה createEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8e7d6c";

const emailTemplate: CreateEmailTemplateBody = {
  name: "Account Activation",
  subject: "Activate Your New Account",
  htmlContent: "<p>Welcome! Please click <a href=\"\{{activationLink}}\">here</a> to activate.</p>",
  // שדות אופציונליים כמו textContent, isActive מוסרים כדי להציג פרמטרים אופציונליים
};

const result: CreateEmailTemplateResponse1 = await createEmailTemplate(
  tenantId,
  emailTemplate
);

console.log(result);
[inline-code-end]