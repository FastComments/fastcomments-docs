req
tenantId
afterId

## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| afterId | string | Όχι |  |
| limit | number | Όχι |  |
| tags | Array<string> | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPosts200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getFeedPosts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_74321";
const afterId: string = "post_20250610_9b2f3";
const limit: number = 25;
const tags: Array<string> = ["product-updates", "announcements"];

const response: GetFeedPosts200Response = await getFeedPosts(tenantId, afterId, limit, tags);
[inline-code-end]

---