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

Retourne : [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple d'utilisation de updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_corp_01";
const id: string = "comment_20260325_4592";
const updatableCommentParams: UpdatableCommentParams = {
  body: "Updated the response to include a link to the RFC and fixed a typo in the second paragraph.",
  editedByUserId: "user_8721",
  isVisible: true
};
const contextUserId: string = "user_8721";
const doSpamCheck: boolean = true;
const isLive: boolean = true;
const result: FlagCommentPublic200Response = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]

---