## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | 是 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 回應

返回：[`SetCommentTextResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentTextResponse1.ts)

## 範例

[inline-code-attrs-start title = 'setCommentText 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const commentId: string = 'cmt_98765';
  const broadcastId: string = 'brd_112233';
  const commentTextUpdateRequest: CommentTextUpdateRequest = {
    text: 'Updated comment with @john.doe and #important',
    mentions: [{ userId: 'user_001', username: 'john.doe' }],
    hashtags: [{ tag: 'important' }],
  };
  const editKey: string = 'edit_abc123';
  const result: SetCommentTextResponse1 = await setCommentText(
    tenantId,
    commentId,
    broadcastId,
    commentTextUpdateRequest,
    editKey,
  );
  console.log(result);
})();
[inline-code-end]