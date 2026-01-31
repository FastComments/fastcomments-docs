## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | Array<string> | Yes |  |
| sso | string | No |  |

## Response

Returns: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsStats200Response.ts)

## Example

[inline-code-attrs-start title = 'getFeedPostsStats Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f2b1c';
const postIds: string[] = ['article-202501', 'article-202502'];
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const statsWithSso: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds, ssoToken);
const statsWithoutSso: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds);
[inline-code-end]
