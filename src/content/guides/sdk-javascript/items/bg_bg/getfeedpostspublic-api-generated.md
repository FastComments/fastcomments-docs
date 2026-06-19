req
tenantId
afterId

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| afterId | string | Не |  |
| limit | number | Не |  |
| tags | Array<string> | Не |  |
| sso | string | Не |  |
| isCrawler | boolean | Не |  |
| includeUserInfo | boolean | Не |  |

## Отговор

Връща: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicFeedPostsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getFeedPostsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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