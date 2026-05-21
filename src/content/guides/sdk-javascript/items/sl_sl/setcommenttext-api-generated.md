## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| broadcastId | string | Da |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Da |  |
| editKey | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentText200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer setCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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