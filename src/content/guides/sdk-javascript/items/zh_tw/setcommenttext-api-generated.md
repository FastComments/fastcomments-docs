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

回傳: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentText200Response.ts)

## 範例

[inline-code-attrs-start title = 'setCommentText 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-42';
const commentId: string = 'cmt-8932';
const broadcastId: string = 'brd-2023-07';
const updateRequest: CommentTextUpdateRequest = {
  text: 'Updated comment text for the product launch — congrats team!',
  mentions: [{ userId: 'user-17', displayName: 'Ava Nguyen' }] as CommentUserMentionInfo[],
  hashtags: [{ tag: 'ProductLaunch' }] as CommentUserHashTagInfo[]
};
const editKey: string = 'edtk-9f7b';
const sso: string = 'sso-token-abc123';
const result: SetCommentText200Response = await setCommentText(tenantId, commentId, broadcastId, updateRequest, editKey, sso);
[inline-code-end]

---