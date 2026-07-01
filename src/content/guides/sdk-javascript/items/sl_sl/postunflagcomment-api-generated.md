## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| commentId | string | Da |  |
| broadcastId | string | Ne |  |
| tenantId | string | Ne |  |
| sso | string | Ne |  |

## Odziv

Vrne: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Primer

[inline-code-attrs-start title = 'postUnFlagComment Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // ID komentarja
    "brd_67890",          // broadcastId (neobvezno)
    "tenant_abc",         // tenantId (neobvezno)
    "sso_user_token_789"  // sso (neobvezno)
  );
};
[inline-code-end]