## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Yanıt

Döndürür: [`DeleteV1PageReactResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV1PageReactResponse.ts)

## Örnek

[inline-code-attrs-start title = 'deleteV1PageReact Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const urlId: string = "article-2024-06-01";

const response: DeleteV1PageReactResponse = await deleteV1PageReact(tenantId, urlId);
console.log(response);
[inline-code-end]