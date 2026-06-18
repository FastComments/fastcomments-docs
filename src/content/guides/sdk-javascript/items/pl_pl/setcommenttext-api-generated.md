## Parametry

| Name | Type | Required | Description |
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

[inline-code-attrs-start title = 'Przykład użycia setCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f9a2b'
const commentId: string = 'cmt-8421'
const broadcastId: string = 'brd-2026-06-15'
const commentTextUpdateRequest: CommentTextUpdateRequest = { text: 'Updated comment text to clarify the schedule.', mentions: [], hashtags: [] }
const editKey: string = 'editkey_9b12'
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso.signature'
const result: SetCommentText200Response = await setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso)
[inline-code-end]

---