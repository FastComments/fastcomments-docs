req
tenantId
afterId

## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| afterId | string | Ne |  |
| limit | number | Ne |  |
| tags | Array<string> | Ne |  |
| sso | string | Ne |  |
| isCrawler | boolean | Ne |  |
| includeUserInfo | boolean | Ne |  |

## Odgovor

Vraća: [`GetFeedPostsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublicResponse.ts)

## Primer

[inline-code-attrs-start title = 'getFeedPostsPublic Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = "tenant_12345";
  const afterId: string = "post_9876";
  const limit: number = 20;
  const tags: string[] = ["news", "tech"];
  const sso: string = "userToken123";
  const isCrawler: boolean = false;
  const includeUserInfo: boolean = true;
  const response: GetFeedPostsPublicResponse = await getFeedPostsPublic(
    tenantId,
    afterId,
    limit,
    tags,
    sso,
    isCrawler,
    includeUserInfo
  );
}
example();
[inline-code-end]

---