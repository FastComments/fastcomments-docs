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
const tenantId: string = "tenant_7a9f2b3d";

const createEmailTemplateBody: CreateEmailTemplateBody = {
  name: "Comment Notification",
  subject: "New comment on your article: {{postTitle}}",
  htmlBody: "<p>{{commenterName}} left a comment:</p><blockquote>{{commentText}}</blockquote>",
  enabled: true,
  defaultLocale: "en-US",
  metadata: { createdBy: "admin@example.com", purpose: "notify_comment" } // optional extra data
};

const result: CreateEmailTemplate200Response = await createEmailTemplate(tenantId, createEmailTemplateBody);
[inline-code-end]
