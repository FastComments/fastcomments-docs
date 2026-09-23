## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postIds | Array<string> | 是 |  |
| sso | string | 否 |  |

## 回應

返回：[`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FeedPostsStatsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getFeedPostsStats 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-9f8b7c6d";
const postIds: string[] = ["post-1a2b3c", "post-4d5e6f", "post-7g8h9i"];
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
const feedStats: FeedPostsStatsResponse = await getFeedPostsStats(tenantId, postIds, ssoToken);
[inline-code-end]

---