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
const tenantId: string = "tenant_acme_001";
const commentId: string = "comment_9f8b7c";
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: "abusive_language",
  durationHours: 48,
  notifyCommenter: true,
  moderatorId: "moderator_42"
};
const sso: string = "sso_jwt_token_example_eyJhbGciOiJIUzI1Ni";
const result: BlockFromCommentPublic200Response = await blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]
