## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postIds | Array<string> | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FeedPostsStatsResponse.ts)

## 예시

[inline-code-attrs-start title = 'getFeedPostsStats 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-9f8b7c6d";
const postIds: string[] = ["post-1a2b3c", "post-4d5e6f", "post-7g8h9i"];
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
const feedStats: FeedPostsStatsResponse = await getFeedPostsStats(tenantId, postIds, ssoToken);
[inline-code-end]

---