## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | Array<string> | Yes |  |
| sso | string | No |  |

## 响应

Returns: [`GetFeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsStatsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getFeedPostsStats 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const postIds: string[] = ["post_a1b2c3", "post_d4e5f6"];
    const ssoToken: string = "sso_abcdef123456";

    const statsWithoutSso: GetFeedPostsStatsResponse = await getFeedPostsStats(tenantId, postIds);
    const statsWithSso: GetFeedPostsStatsResponse = await getFeedPostsStats(tenantId, postIds, ssoToken);
}

runExample();
[inline-code-end]