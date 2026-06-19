## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| postIds | Array<string> | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FeedPostsStatsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getFeedPostsStats 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-corp-tenant-001';
  const postIds: Array<string> = ['post_4d2f1a', 'post_7b9c3e', 'post_b8a0d4'];
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI0MjMifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
  const statsWithSso: FeedPostsStatsResponse = await getFeedPostsStats(tenantId, postIds, sso);
  const statsWithoutSso: FeedPostsStatsResponse = await getFeedPostsStats(tenantId, postIds);
  console.log(statsWithSso, statsWithoutSso);
})();
[inline-code-end]