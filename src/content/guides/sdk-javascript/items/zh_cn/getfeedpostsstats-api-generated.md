## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | Array<string> | Yes |  |
| sso | string | No |  |

## 响应

返回: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsStats200Response.ts)

## 示例

[inline-code-attrs-start title = 'getFeedPostsStats 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_98765";
const postIds: string[] = ["post_a1b2c3", "post_d4e5f6"];
const ssoToken: string = "sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

const statsWithoutSSO: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds);
const statsWithSSO: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds, ssoToken);
[inline-code-end]

---