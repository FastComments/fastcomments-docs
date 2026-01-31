## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentText200Response.ts)

## Example

[inline-code-attrs-start title = 'setCommentText Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9b7f2c";
const commentId: string = "cmt_8421";
const broadcastId: string = "brd_live_2026";
const mention: CommentUserMentionInfo = { userId: "user_314", displayName: "Alex Chen" };
const hashtag: CommentUserHashTagInfo = { tag: "product" };
const commentTextUpdateRequest: CommentTextUpdateRequest = {
  text: "Updated: confirmed the new release schedule — tagging Alex for details.",
  mentions: [mention],
  hashtags: [hashtag]
};
const editKey: string = "edit_7f9a2c";
const sso: string = "sso_jwt_token_example_2026";
const result: SetCommentText200Response = await setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso);
[inline-code-end]
