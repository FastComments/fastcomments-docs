## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Yes |  |

## Response

Returns: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateEmailTemplate200Response.ts)

## Example

[inline-code-attrs-start title = 'createEmailTemplate Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-42";
const createEmailTemplateBody: CreateEmailTemplateBody = {
  templateId: "welcome_v1",
  name: "New User Welcome",
  subject: "Welcome to Acme Corp — next steps",
  htmlBody: "<p>Hi \{{firstName}}, welcome to Acme Corp! Use code <strong>GETSTARTED</strong>.</p>",
  description: "Onboarding welcome email for newly registered users", // optional parameter demonstrated
  isActive: true,
  previewText: "Start using your Acme account today" // another optional field
};
const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]
