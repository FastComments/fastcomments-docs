## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| postIds | Array<string> | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrača: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FeedPostsStatsResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getFeedPostsStats'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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