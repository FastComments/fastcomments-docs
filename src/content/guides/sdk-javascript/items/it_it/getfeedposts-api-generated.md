req
tenantId
afterId

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| afterId | string | No |  |
| limit | number | No |  |
| tags | Array<string> | No |  |

## Risposta

Restituisce: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-01';
const afterId: string | undefined = 'post_20250601_89';
const limit: number = 20;
const tags: string[] = ['product-update', 'engineering'];
const result: GetFeedPostsResponse = await getFeedPosts(tenantId, afterId, limit, tags);
[inline-code-end]

---