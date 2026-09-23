## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentBanStatusResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentBanStatus Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_987654";
  const ssoToken: string = "sso_user_abc";

  const statusWithSso: GetCommentBanStatusResponse = await getCommentBanStatus(tenantId, commentId, ssoToken);
  const statusWithoutSso: GetCommentBanStatusResponse = await getCommentBanStatus(tenantId, commentId);
})();
[inline-code-end]