## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Risposta

Restituisce: [`GetV1PageLikesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV1PageLikesResponse.ts)

## Esempio

[inline-code-attrs-start title = 'getV1PageLikes Esempio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPageLikes() {
  const tenantId: string = "acme-corp-tenant-42";
  const urlId: string = "blog-post-2024-06-typescript-best-practices";

  const likes: GetV1PageLikesResponse = await getV1PageLikes(tenantId, urlId);
  console.log(likes);
}
[inline-code-end]

---