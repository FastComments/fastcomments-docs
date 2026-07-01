req
tenantId
afterId

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|------------|--------------|
| tenantId | string | Ja |  |
| afterId | string | Nee |  |
| limit | number | Nee |  |
| tags | Array<string> | Nee |  |
| sso | string | Nee |  |
| isCrawler | boolean | Nee |  |
| includeUserInfo | boolean | Nee |  |

## Respons

Retourneert: [`GetFeedPostsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublicResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPostsPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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