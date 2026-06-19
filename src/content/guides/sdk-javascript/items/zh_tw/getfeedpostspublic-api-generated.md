---
req
tenantId
afterId

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| afterId | string | 否 |  |
| limit | number | 否 |  |
| tags | Array<string> | 否 |  |
| sso | string | 否 |  |
| isCrawler | boolean | 否 |  |
| includeUserInfo | boolean | 否 |  |

## 回應

回傳: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicFeedPostsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getFeedPostsPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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