## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Oui |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |

## Réponse

Retourne : [`UnBlockUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockUserFromCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple unBlockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUnblock() {
  const tenantId: string = "acme-corp-tenant";
  const commentId: string = "cmt_9f8b7a6d";

  const params: UnBlockFromCommentParams = {
    reason: "User resolved the issue",
    notifyUser: true
  };

  const userId: string = "usr_12345";

  const result: UnBlockUserFromCommentResponse = await unBlockUserFromComment(
    tenantId,
    commentId,
    params,
    userId
    // anonUserId omis
  );

  console.log(result);
}
demoUnblock();
[inline-code-end]