req
tenantId
afterId

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| afterId | string | Ne |  |
| limit | number | Ne |  |
| tags | Array<string> | Ne |  |
| sso | string | Ne |  |
| isCrawler | boolean | Ne |  |
| includeUserInfo | boolean | Ne |  |

## Odgovor

Vraća: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublic200Response.ts)

## Primer

[inline-code-attrs-start title = 'getFeedPostsPublic Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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