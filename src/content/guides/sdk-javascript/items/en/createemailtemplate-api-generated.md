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
const tenantId: string = "tenant_9b3f2a";
const createEmailTemplateBody: CreateEmailTemplateBody = {
  name: "Welcome to Redwood Co.",
  subject: "Welcome aboard — getting started with your account",
  htmlContent: "<p>Hi \{{firstName}},</p><p>Welcome to Redwood Co. We're glad you're here.</p>",
  isActive: true,
  description: "Customer onboarding welcome email" // optional field demonstrated
};
const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]
