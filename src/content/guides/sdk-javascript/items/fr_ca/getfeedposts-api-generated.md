req
tenantId
afterId

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| afterId | string | Non |  |
| limit | number | Non |  |
| tags | Array<string> | Non |  |

## Réponse

Renvoie : [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-01';
const afterId: string | undefined = 'post_20250601_89';
const limit: number = 20;
const tags: string[] = ['product-update', 'engineering'];
const result: GetFeedPostsResponse = await getFeedPosts(tenantId, afterId, limit, tags);
[inline-code-end]

---