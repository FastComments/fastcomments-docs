## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| broadcastId | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'unLockComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUnlock() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";
  const broadcastId: string = "brd_987654321";
  const ssoToken: string = "sso_abcdef123456";

  const resultWithoutSso: APIEmptyResponse = await unLockComment(tenantId, commentId, broadcastId);
  const resultWithSso: APIEmptyResponse = await unLockComment(tenantId, commentId, broadcastId, ssoToken);
}
[inline-code-end]