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

[inline-code-attrs-start title = 'flagComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9a1b2c';
const commentId: string = 'comment_20260109_001';
const userId: string = 'user_8347';
const anonUserId: string = 'anon_abc123';

const responseWithUser: FlagComment200Response = await flagComment(tenantId, commentId, userId);
const responseWithAnon: FlagComment200Response = await flagComment(tenantId, commentId, undefined, anonUserId);
[inline-code-end]
