
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
const tenantId: string = 'tenant_acme_01';
const afterId: string | undefined = 'post_20251120_001';
const tags: string[] = ['product-update', 'security'];
const result: GetFeedPosts200Response = await getFeedPosts(tenantId, afterId, undefined, tags);
console.log(result);
[inline-code-end]
