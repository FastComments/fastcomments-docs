请求
tenantId
afterId

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| afterId | string | 否 |  |
| limit | number | 否 |  |
| tags | Array<string> | 否 |  |
| sso | string | 否 |  |
| isCrawler | boolean | 否 |  |
| includeUserInfo | boolean | 否 |  |

## 响应

返回: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'getFeedPostsPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const afterId: string = 'fp_20260301_042';
const limit: number = 25;
const tags: Array<string> = ['technology', 'announcement'];
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoiamRvZSJ9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const isCrawler: boolean = false;
const includeUserInfo: boolean = true;
const response: GetFeedPostsPublic200Response = await getFeedPostsPublic(tenantId, afterId, limit, tags, sso, isCrawler, includeUserInfo);
[inline-code-end]

---