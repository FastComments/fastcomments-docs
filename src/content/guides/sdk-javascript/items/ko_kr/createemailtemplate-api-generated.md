## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createEmailTemplateBody | CreateEmailTemplateBody | 예 |  |

## 응답

반환: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## 예제

[inline-code-attrs-start title = 'createEmailTemplate 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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