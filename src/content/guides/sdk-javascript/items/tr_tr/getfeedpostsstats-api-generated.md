---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| postIds | Array<string> | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FeedPostsStatsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getFeedPostsStats Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-9f8b7c6d";
const postIds: string[] = ["post-1a2b3c", "post-4d5e6f", "post-7g8h9i"];
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
const feedStats: FeedPostsStatsResponse = await getFeedPostsStats(tenantId, postIds, ssoToken);
[inline-code-end]

---