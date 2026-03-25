## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| broadcastId | string | Да |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Да |  |
| editKey | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentText200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример setCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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