## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Одговор

Враћа: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'postUnFlagComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // commentId
    "brd_67890",          // broadcastId (опционално)
    "tenant_abc",         // tenantId (опционално)
    "sso_user_token_789"  // sso (опционално)
  );
};
[inline-code-end]

---