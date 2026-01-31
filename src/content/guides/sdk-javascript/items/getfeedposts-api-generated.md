
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
const tenantId: string = 'acme-tenant-9f2';
const afterId: string = 'feedpost_000123';
const limit: number = 20;
const tags: Array<string> = ['product-update', 'release-notes'];
const response: GetFeedPosts200Response = await getFeedPosts(tenantId, afterId, limit, tags);
[inline-code-end]
