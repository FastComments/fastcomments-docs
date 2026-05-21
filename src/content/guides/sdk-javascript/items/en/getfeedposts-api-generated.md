
req
tenantId
afterId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| afterId | string | No |  |
| limit | number | No |  |
| tags | Array<string> | No |  |

## Response

Returns: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPosts200Response.ts)

## Example

[inline-code-attrs-start title = 'getFeedPosts Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8b7f2c';
const limit: number = 20;
const tags: Array<string> = ['product-updates', 'community'];
const firstPage: GetFeedPosts200Response = await getFeedPosts(tenantId, undefined, limit, tags);
const afterId: string = 'post_9c3d2e';
const secondPage: GetFeedPosts200Response = await getFeedPosts(tenantId, afterId, limit);
[inline-code-end]
