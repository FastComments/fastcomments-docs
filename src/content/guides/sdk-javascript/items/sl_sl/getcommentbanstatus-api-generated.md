## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentBanStatusResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getCommentBanStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_987654";
  const ssoToken: string = "sso_user_abc";

  const statusWithSso: GetCommentBanStatusResponse = await getCommentBanStatus(tenantId, commentId, ssoToken);
  const statusWithoutSso: GetCommentBanStatusResponse = await getCommentBanStatus(tenantId, commentId);
})();
[inline-code-end]