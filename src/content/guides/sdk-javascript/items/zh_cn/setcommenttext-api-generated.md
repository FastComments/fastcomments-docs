## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | 是 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPISetCommentTextResponse.ts)

## 示例

[inline-code-attrs-start title = 'setCommentText 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42f2a9';
const commentId: string = 'cmt_9b7d3e';
const broadcastId: string = 'brd_live_2026_06_19';
const editKey: string = 'edk_3f8d2c4a9';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature';

const mention: CommentUserMentionInfo = { userId: 'user_789', displayName: 'Jordan Mills' };
const hashtag: CommentUserHashTagInfo = { tag: 'product-launch' };

const commentTextUpdateRequest: CommentTextUpdateRequest = {
  text: 'Updated: clarified the timeline and fixed a typo in the earlier comment.',
  mentions: [mention],
  hashtags: [hashtag]
};

const result: PublicAPISetCommentTextResponse = await setCommentText(
  tenantId,
  commentId,
  broadcastId,
  commentTextUpdateRequest,
  editKey,
  sso
);
[inline-code-end]

---