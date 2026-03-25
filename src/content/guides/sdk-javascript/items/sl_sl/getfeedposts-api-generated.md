req
tenantId
afterId

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| afterId | string | Ne |  |
| limit | number | Ne |  |
| tags | Array<string> | Ne |  |

## Odgovor

Vrne: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPosts200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const initialPage: GetFeedPosts200Response = await getFeedPosts('tenant_9f1b3d', undefined, 20, ['sports', 'local']);
const nextPage: GetFeedPosts200Response = await getFeedPosts('tenant_9f1b3d', 'post_abc123', 20, ['sports', 'local']);
[inline-code-end]

---