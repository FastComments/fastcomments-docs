## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## Example

[inline-code-attrs-start title = 'unFlagComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments_tenant_001';
const commentId: string = 'comment_55f3a2';
const userId: string = 'user_7890';
const anonUserId: string = 'anon_1a2b3c';
const result: FlagComment200Response = await unFlagComment(tenantId, commentId, userId, anonUserId);
[inline-code-end]
