req
tenantId
afterId

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| afterId | string | לא |  |
| limit | number | לא |  |
| tags | Array<string> | לא |  |
| sso | string | לא |  |
| isCrawler | boolean | לא |  |
| includeUserInfo | boolean | לא |  |

## תגובה

מחזיר: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getFeedPostsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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