## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## Response

Returns: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'blockFromCommentPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_newsroom_42';
const commentId: string = 'cmt_9f1e4b7a';
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: 'Repeated promotional links across multiple threads',
  blockDurationHours: 72,
  notifyModerator: true
};
const sso: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload';
const response: BlockFromCommentPublic200Response = await blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]
