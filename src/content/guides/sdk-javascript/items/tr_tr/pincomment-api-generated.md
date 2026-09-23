## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| broadcastId | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeCommentPinStatusResponse.ts)

## Örnek

[inline-code-attrs-start title = 'pinComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoPinComment() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_54321";

  const resultWithoutSso: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId);
  const ssoToken: string = "sso_abcde12345";
  const resultWithSso: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId, ssoToken);
}

demoPinComment();
[inline-code-end]