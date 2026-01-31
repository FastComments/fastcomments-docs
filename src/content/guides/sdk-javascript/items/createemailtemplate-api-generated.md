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
const tenantId: string = "acme-tenant-001";
const configParameters: CustomConfigParameters = {
  allowUnsubscribe: true,
  unsubscribeUrl: "https://acme.example.com/unsubscribe"
};
const templateBody: CreateEmailTemplateBody = {
  name: "Comment Reply Notification",
  subject: "New reply to your comment on Acme Blog",
  fromName: "Acme Blog",
  fromAddress: "no-reply@acme.example.com",
  htmlBody: "<p>Hi {{commenterName}}, a new reply was posted to your comment.</p>",
  plainBody: "Hi {{commenterName}}, a new reply was posted to your comment.",
  enabled: true,
  configParameters
};
const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, templateBody);
[inline-code-end]
