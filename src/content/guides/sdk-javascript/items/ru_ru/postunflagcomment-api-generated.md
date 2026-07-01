## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Ответ

Возвращает: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример postUnFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // ID комментария
    "brd_67890",          // broadcastId (необязательно)
    "tenant_abc",         // tenantId (необязательно)
    "sso_user_token_789"  // sso (необязательно)
  );
};
[inline-code-end]