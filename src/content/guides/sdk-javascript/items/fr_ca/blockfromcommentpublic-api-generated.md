## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Oui |  |
| sso | string | Non |  |

## Réponse

Retourne : [`BlockFromCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublicResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple blockFromCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_9f8b7c";
  const commentId: string = "cmt_1234567890";
  const blockParams: PublicBlockFromCommentParams = {
    reason: "spam",
    durationHours: 24,
  };
  const ssoToken: string = "sso_ABCDEF123456";

  const responseWithSso: BlockFromCommentPublicResponse = await blockFromCommentPublic(
    tenantId,
    commentId,
    blockParams,
    ssoToken
  );

  const responseWithoutSso: BlockFromCommentPublicResponse = await blockFromCommentPublic(
    tenantId,
    commentId,
    blockParams
  );

  console.log(responseWithSso, responseWithoutSso);
}
demo();
[inline-code-end]