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
const tenantId: string = 'tenant_00123';
const id: string = 'tmpl_4f2a9b';
const customConfig: CustomConfigParameters = { maxRetries: 2, sendDelayMinutes: 0 };
const updateEmailTemplateBody: UpdateEmailTemplateBody = {
  name: 'Account Settings Update',
  subject: 'Your account settings were changed',
  htmlBody: '<p>Your account settings were successfully updated.</p>',
  isActive: true,
  senderName: 'Customer Support',
  replyTo: 'support@company.com',
  customConfig
};
const result: FlagCommentPublic200Response = await updateEmailTemplate(tenantId, id, updateEmailTemplateBody);
[inline-code-end]
