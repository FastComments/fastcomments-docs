## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| broadcastId | string | Tak |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Tak |  |
| editKey | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentText200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład setCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6721f4';
const commentId: string = 'cmt_9a3b2d';
const broadcastId: string = 'live_2026_03_25';
const editKey: string = 'edit_k_4f7b9';
const sso: string = 'sso_tok_eyJhbGciOiJIUzI1';
const commentTextUpdateRequest: CommentTextUpdateRequest = {
  text: 'Updated to clarify the timeline and link the relevant docs.',
  mentions: [{ userId: 'user_102', displayName: 'Alex Rivera' }],
  hashtags: [{ tag: 'product-update' }]
};
const result: SetCommentText200Response = await setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso);
[inline-code-end]

---