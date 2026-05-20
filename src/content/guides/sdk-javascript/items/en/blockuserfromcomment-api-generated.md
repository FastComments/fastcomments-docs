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
const tenantId: string = 'tenant_abc123';
const commentId: string = 'comment_98765';
const blockFromCommentParams: BlockFromCommentParams = { reason: 'abusive_language', durationDays: 30 };
const userId: string = 'user_42';
const result: BlockFromCommentPublic200Response = await blockUserFromComment(tenantId, commentId, blockFromCommentParams, userId);
[inline-code-end]
