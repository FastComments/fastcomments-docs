## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | Yes |  |
| title | string | No |  |

## Yanıt

Döndürür: [`CreateV2PageReactResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV2PageReactResponse.ts)

## Örnek

[inline-code-attrs-start title = 'createV2PageReact Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const urlId: string = "blog/posts/fastcomments-integration";
  const pageId: string = "page-12345";
  const title: string = "FastComments API Integration Guide";

  const responseWithoutTitle: CreateV2PageReactResponse = await createV2PageReact(tenantId, urlId, pageId);
  const responseWithTitle: CreateV2PageReactResponse = await createV2PageReact(tenantId, urlId, pageId, title);

  console.log(responseWithoutTitle, responseWithTitle);
})();
[inline-code-end]