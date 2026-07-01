## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Réponse

Retourne : [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple postUnFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // commentId
    "brd_67890",          // broadcastId (optionnel)
    "tenant_abc",         // tenantId (optionnel)
    "sso_user_token_789"  // sso (optionnel)
  );
};
[inline-code-end]