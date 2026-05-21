## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 예 |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | 예 |  |
| editKey | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentText200Response.ts)

## 예제

[inline-code-attrs-start title = 'setCommentText 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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