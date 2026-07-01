## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Odpowiedź

Zwraca: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład postUnFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // ID komentarza
    "brd_67890",          // ID transmisji (opcjonalnie)
    "tenant_abc",         // ID najemcy (opcjonalnie)
    "sso_user_token_789"  // sso (opcjonalnie)
  );
};
[inline-code-end]