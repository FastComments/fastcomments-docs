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
const tenantId: string = 'tenant_acme_42';
const id: string = 'emailTemplate_pwd_reset_9f3c';
const updateEmailTemplateBody: UpdateEmailTemplateBody = {
  name: 'Password Reset Notification',
  subject: 'Reset your ACME password',
  html: '<p>Click <a href="\{{reset_link}}">here</a> to reset your password.</p>'
} as UpdateEmailTemplateBody;
const result: FlagCommentPublic200Response = await updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
[inline-code-end]
