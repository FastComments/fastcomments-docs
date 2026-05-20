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
const tenantId: string = 'acme-corp-42';
const moderatorId: string = 'mod_9374';
const sendEmail: string | undefined = 'notifications@acme-corp.com';
const result: FlagCommentPublic200Response = await deleteModerator(tenantId, moderatorId, sendEmail);

[inline-code-end]
