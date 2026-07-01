## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| commentId | string | Oui |  |
| broadcastId | string | Non |  |
| tenantId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple postUnFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // identifiant du commentaire
    "brd_67890",          // identifiant de diffusion (facultatif)
    "tenant_abc",         // identifiant du locataire (facultatif)
    "sso_user_token_789"  // sso (facultatif)
  );
};
[inline-code-end]