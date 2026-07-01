## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| commentId | string | Да |  |
| broadcastId | string | Нет |  |
| tenantId | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'postUnFlagComment пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // commentId
    "brd_67890",          // broadcastId (опционально)
    "tenant_abc",         // tenantId (опционально)
    "sso_user_token_789"  // sso (опционально)
  );
};
[inline-code-end]