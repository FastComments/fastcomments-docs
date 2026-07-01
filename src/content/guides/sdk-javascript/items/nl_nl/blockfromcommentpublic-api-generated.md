## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`BlockFromCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublicResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'blockFromCommentPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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