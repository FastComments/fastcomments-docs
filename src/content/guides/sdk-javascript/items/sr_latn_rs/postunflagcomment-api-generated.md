## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| commentId | string | Da |  |
| broadcastId | string | Ne |  |
| tenantId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Primer

[inline-code-attrs-start title = 'postUnFlagComment Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // ID komentara
    "brd_67890",          // broadcastId (opcionalno)
    "tenant_abc",         // tenantId (opcionalno)
    "sso_user_token_789"  // sso (opcionalno)
  );
};
[inline-code-end]