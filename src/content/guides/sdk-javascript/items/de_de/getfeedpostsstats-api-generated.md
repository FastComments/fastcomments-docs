## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| postIds | Array<string> | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FeedPostsStatsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getFeedPostsStats Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-9f8b7c6d";
const postIds: string[] = ["post-1a2b3c", "post-4d5e6f", "post-7g8h9i"];
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
const feedStats: FeedPostsStatsResponse = await getFeedPostsStats(tenantId, postIds, ssoToken);
[inline-code-end]