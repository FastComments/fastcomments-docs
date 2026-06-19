verplicht
tenantId
afterId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nee |  |
| limit | number | Nee |  |
| tags | Array<string> | Nee |  |
| sso | string | Nee |  |
| isCrawler | boolean | Nee |  |
| includeUserInfo | boolean | Nee |  |

## Antwoord

Geeft terug: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicFeedPostsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPostsPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "fc_tenant_12345";
  const afterId: string = "feedPost_98765";
  const limit: number = 20;
  const tags: Array<string> = ["announcement", "product"];
  const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example";
  const isCrawler: boolean = false;
  const includeUserInfo: boolean = true;

  const response: PublicFeedPostsResponse = await getFeedPostsPublic(tenantId, afterId, limit, tags, sso, isCrawler, includeUserInfo);
  console.log(response);
})();
[inline-code-end]

---