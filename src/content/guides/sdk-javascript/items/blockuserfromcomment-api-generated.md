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
const tenantId: string = 'tenant_6a1f4b2c';
const id: string = 'cmt_9f2d3a7b';
const blockFromCommentParams: BlockFromCommentParams = {
  reason: 'spam',
  durationDays: 30,
  notifyModerator: true
};
const userId: string = 'user_84b2a1c';
const result: BlockFromCommentPublic200Response = await blockUserFromComment(tenantId, id, blockFromCommentParams, userId);
[inline-code-end]
