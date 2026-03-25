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

Renvoie : [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPosts200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const initialPage: GetFeedPosts200Response = await getFeedPosts('tenant_9f1b3d', undefined, 20, ['sports', 'local']);
const nextPage: GetFeedPosts200Response = await getFeedPosts('tenant_9f1b3d', 'post_abc123', 20, ['sports', 'local']);
[inline-code-end]