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
const tenantId: string = 'org-8421';
const postIds: string[] = [
  'c9a1e3f2-7b6d-4a11-9c2f-000000000001',
  'd4b2c3a5-8e7f-4b22-9d3e-000000000002'
];
const statsWithoutSso: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds);
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI0NTY3IiwiaWF0IjoxNjAwMDAwMDB9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const statsWithSso: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds, ssoToken);
[inline-code-end]
