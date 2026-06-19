## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postIds | Array<string> | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FeedPostsStatsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getFeedPostsStats 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---