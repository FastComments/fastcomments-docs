## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Отговор

Връща: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'postUnFlagComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // commentId
    "brd_67890",          // broadcastId (по избор)
    "tenant_abc",         // tenantId (по избор)
    "sso_user_token_789"  // sso (по избор)
  );
};
[inline-code-end]