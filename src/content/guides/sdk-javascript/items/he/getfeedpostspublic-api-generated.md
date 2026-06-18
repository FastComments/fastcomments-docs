---
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

[inline-code-attrs-start title = 'דוגמה של getFeedPostsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_01';
  const afterId: string = 'post_20250610_842';
  const limit: number = 25;
  const tags: string[] = ['news', 'technology'];
  const sso: string = 'sso_jwt_eyJhbGciOiJIUzI1Ni';
  const isCrawler: boolean = false;
  const includeUserInfo: boolean = true;

  const response: GetFeedPostsPublic200Response = await getFeedPostsPublic(
    tenantId,
    afterId,
    limit,
    tags,
    sso,
    isCrawler,
    includeUserInfo
  );

  console.log(response);
})();
[inline-code-end]

---