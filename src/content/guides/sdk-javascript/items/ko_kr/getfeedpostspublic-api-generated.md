req
tenantId
afterId

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| afterId | string | 아니오 |  |
| limit | number | 아니오 |  |
| tags | Array<string> | 아니오 |  |
| sso | string | 아니오 |  |
| isCrawler | boolean | 아니오 |  |
| includeUserInfo | boolean | 아니오 |  |

## 응답

반환: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'getFeedPostsPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---