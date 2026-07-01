## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| commentId | string | Oui |  |
| tenantId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`GetBanUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBanUsersFromCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'getBanUsersFromComment Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetBanUsers() {
  const commentId: string = "cmt_5f8e3a9b2d";
  const tenantId: string = "tenant_42";
  const sso: string = "sso_token_abc123";

  // Appel avec tous les paramètres
  const fullResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId, tenantId, sso);
  console.log(fullResult);

  // Appel avec uniquement le paramètre requis
  const minimalResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId);
  console.log(minimalResult);
}

demoGetBanUsers();
[inline-code-end]

---