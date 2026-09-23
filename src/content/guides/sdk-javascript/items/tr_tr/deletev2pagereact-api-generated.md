## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| id | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Örnek

[inline-code-attrs-start title = 'deleteV2PageReact Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeleteExamples() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_98765";
  const commentId: string = "comment_abcde";

  // Opsiyonel sso olmadan çağır
  const resultWithoutSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId);

  // Opsiyonel sso ile çağır
  const ssoToken: string = "sso_token_xyz";
  const resultWithSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId, ssoToken);
}
[inline-code-end]