## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|---------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |

## Respons

Returns: [`GetV1PageLikesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikesResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getV1PageLikes Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPageLikes() {
  const tenantId: string = "acme-corp-tenant-42";
  const urlId: string = "blog-post-2024-06-typescript-best-practices";

  const likes: GetV1PageLikesResponse = await getV1PageLikes(tenantId, urlId);
  console.log(likes);
}
[inline-code-end]