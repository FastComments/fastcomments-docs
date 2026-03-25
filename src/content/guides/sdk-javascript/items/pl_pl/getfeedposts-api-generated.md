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

Zwraca: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPosts200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const initialPage: GetFeedPosts200Response = await getFeedPosts('tenant_9f1b3d', undefined, 20, ['sports', 'local']);
const nextPage: GetFeedPosts200Response = await getFeedPosts('tenant_9f1b3d', 'post_abc123', 20, ['sports', 'local']);
[inline-code-end]

---