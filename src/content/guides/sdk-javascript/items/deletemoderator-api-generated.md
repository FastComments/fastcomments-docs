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
const tenantId: string = 'tenant_9f8b7c6d-001';
const moderatorId: string = 'mod_4e3f2a90-77b3';
const notifyEmail: string = 'jane.doe@acme-corp.com';
const response: FlagCommentPublic200Response = await deleteModerator(tenantId, moderatorId, notifyEmail);
console.log(response);
[inline-code-end]
