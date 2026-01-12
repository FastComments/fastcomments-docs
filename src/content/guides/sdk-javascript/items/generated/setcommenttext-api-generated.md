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
const tenantId: string = "tenant_prod_81f4b3";
const commentId: string = "cmt_9a2f4e7b";
const broadcastId: string = "broadcast_2026_01_12_live";
const mention: CommentUserMentionInfo = { userId: "user_123", displayName: "Jane Doe" };
const hashtag: CommentUserHashTagInfo = { tag: "product-feedback" };
const commentTextUpdateRequest: CommentTextUpdateRequest = {
  text: "Thanks for the update — tagging @Jane Doe for visibility. #product-feedback",
  mentions: [mention],
  hashtags: [hashtag]
};
const editKey: string = "editkey_5f6b7c9d";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature";
const result: SetCommentText200Response = await setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso);
[inline-code-end]
