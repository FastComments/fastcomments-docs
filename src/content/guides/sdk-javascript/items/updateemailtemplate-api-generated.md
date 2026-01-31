## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateEmailTemplate Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_78b2f4';
const id: string = 'email_tpl_4f1a9c';
const updateEmailTemplateBody: UpdateEmailTemplateBody = {
  subject: 'Flagged comment notification',
  html: '<p>A comment was flagged. Review it <a href="https://admin.acme-corp.com/moderation">here</a>.</p>',
  enabled: true,
  senderName: 'Acme Moderation',
  replyTo: 'no-reply@acme-corp.com' // optional parameter demonstrated
};
const result: FlagCommentPublic200Response = await updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
[inline-code-end]
