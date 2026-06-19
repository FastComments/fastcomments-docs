## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`BlockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockSuccess.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de blockFromCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_52b9f3a1";
const commentId: string = "cmt_4f9d2a7b";
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: "spam",
  moderatorId: "mod_783",
  durationMinutes: 1440,
  notifyUser: true
};
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example";
const result: BlockSuccess = await blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]