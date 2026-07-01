## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updatableCommentParams | UpdatableCommentParams | Oui |  |
| contextUserId | string | Non |  |
| doSpamCheck | boolean | Non |  |
| isLive | boolean | Non |  |

## Réponse

Renvoie : [`UpdateCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateCommentResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de mise à jour de commentaire'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";

const updatableCommentParams: UpdatableCommentParams = {
  // champs d'exemple ; la forme réelle dépend de la définition de l'API
  // p. ex., body: "Edited comment content",
};

const contextUserId: string = "user_abcde";
const doSpamCheck: boolean = true;
const isLive: boolean = false;

const result: UpdateCommentResponse = await updateComment(
  tenantId,
  commentId,
  updatableCommentParams,
  contextUserId,
  doSpamCheck,
  isLive
);
[inline-code-end]