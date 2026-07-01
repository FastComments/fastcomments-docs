## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| commentId | string | Oui |  |
| includeEmail | boolean | Non |  |
| includeIP | boolean | Non |  |
| tenantId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // Jeu complet de paramètres
  const commentId: string = "cmt_12345abc";
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const tenantId: string = "tenant_9876";
  const sso: string = "sso_token_xyz";

  const fullResult: GetModerationCommentResponse = await getModerationComment(
    commentId,
    includeEmail,
    includeIP,
    tenantId,
    sso
  );

  // Appel minimal n'utilisant que l'argument requis
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // Utilisez les résultats selon les besoins...
}
[inline-code-end]