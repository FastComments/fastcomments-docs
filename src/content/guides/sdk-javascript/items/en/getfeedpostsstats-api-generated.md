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
async function fetchStats(): Promise<void> {
  const tenantId: string = 'tenant-4c2b1a';
  const postIds: string[] = ['post_7f1e3a', 'post_9b2c6d'];
  const sso: string | undefined = 'sso_tok_9XyZ123';
  const result: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds, sso);
  console.log(result);
}
fetchStats();
[inline-code-end]
