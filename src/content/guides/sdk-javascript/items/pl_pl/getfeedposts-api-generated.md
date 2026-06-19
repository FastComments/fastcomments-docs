req
tenantId
afterId

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| afterId | string | Nie |  |
| limit | number | Nie |  |
| tags | Array<string> | Nie |  |

## Odpowiedź

Zwraca: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-01';
const afterId: string | undefined = 'post_20250601_89';
const limit: number = 20;
const tags: string[] = ['product-update', 'engineering'];
const result: GetFeedPostsResponse = await getFeedPosts(tenantId, afterId, limit, tags);
[inline-code-end]

---