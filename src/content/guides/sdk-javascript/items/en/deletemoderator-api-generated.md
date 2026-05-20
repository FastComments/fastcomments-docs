## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| sendEmail | string | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteModerator Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const moderatorId: string = 'mod_84021';
const sendEmail: string = 'notifications@acme-corp.com';
const resultWithEmail: FlagCommentPublic200Response = await deleteModerator(tenantId, moderatorId, sendEmail);
const resultWithoutEmail: FlagCommentPublic200Response = await deleteModerator(tenantId, moderatorId);
[inline-code-end]
