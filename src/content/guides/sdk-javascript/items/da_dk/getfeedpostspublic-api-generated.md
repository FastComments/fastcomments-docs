req
tenantId
afterId

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nej |  |
| limit | number | Nej |  |
| tags | Array<string> | Nej |  |
| sso | string | Nej |  |
| isCrawler | boolean | Nej |  |
| includeUserInfo | boolean | Nej |  |

## Svar

Returnerer: [`GetFeedPostsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublicResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getFeedPostsPublic Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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