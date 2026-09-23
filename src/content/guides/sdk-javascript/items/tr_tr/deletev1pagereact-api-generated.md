## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Örnek

[inline-code-attrs-start title = 'deleteV1PageReact Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExamples() {
  const tenantId: string = "tenant_9f8b7c6d";
  const urlId: string = "blog/post-2024-06-15";
  const ssoToken: string = "sso_user_42";

  const resultWithSso: CreateV1PageReact = await deleteV1PageReact(tenantId, urlId, ssoToken);
  const resultWithoutSso: CreateV1PageReact = await deleteV1PageReact(tenantId, urlId);
}
[inline-code-end]

---