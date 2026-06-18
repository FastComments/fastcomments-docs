## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| broadcastId | string | Да |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Да |  |
| editKey | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentText200Response.ts)

## Пример

[inline-code-attrs-start title = 'setCommentText Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f9a2b'
const commentId: string = 'cmt-8421'
const broadcastId: string = 'brd-2026-06-15'
const commentTextUpdateRequest: CommentTextUpdateRequest = { text: 'Updated comment text to clarify the schedule.', mentions: [], hashtags: [] }
const editKey: string = 'editkey_9b12'
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso.signature'
const result: SetCommentText200Response = await setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso)
[inline-code-end]