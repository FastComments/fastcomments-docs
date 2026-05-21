## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | はい |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | はい |  |
| editKey | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentText200Response.ts)

## 例

[inline-code-attrs-start title = 'setCommentText の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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