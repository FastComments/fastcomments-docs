
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
| sso | string | No |  |
| isCrawler | boolean | No |  |
| includeUserInfo | boolean | No |  |

## Response

Returns: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'getFeedPostsPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_corp_01";
const afterId: string = "post_20260105_0001";
const limit: number = 25;
const tags: string[] = ["technology", "typescript"];
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature";
const isCrawler: boolean = false;
const includeUserInfo: boolean = true;
const result: GetFeedPostsPublic200Response = await getFeedPostsPublic(tenantId, afterId, limit, tags, sso, isCrawler, includeUserInfo);
[inline-code-end]
