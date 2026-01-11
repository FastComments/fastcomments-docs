## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| blockFromCommentParams | BlockFromCommentParams | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'blockUserFromComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-004';
const commentId: string = 'cmt-9f2a3b4c';
const blockFromCommentParams: BlockFromCommentParams = {
  reason: 'Repeated spam and promotional links',
  durationDays: 30,
  blockReplies: true,
  notifyModerators: true
};
const userId: string = 'user-000123';
const anonUserId: string = 'anon-8f7b2c';
const result: BlockFromCommentPublic200Response = await blockUserFromComment(tenantId, commentId, blockFromCommentParams, userId, anonUserId);
[inline-code-end]
