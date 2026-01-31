## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'unBlockUserFromComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b';
const id: string = 'comment_87a2b';
const unBlockFromCommentParams: UnBlockFromCommentParams = {
  unblockReason: 'appeal accepted by moderation',
  moderatorId: 'mod_13'
} as UnBlockFromCommentParams;
const userId: string = 'user_204';
const anonUserId: string = 'anon_5d2c';
const result: UnBlockCommentPublic200Response = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId, anonUserId);
[inline-code-end]
