req
tenantId
afterId

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| afterId | string | いいえ |  |
| limit | number | いいえ |  |
| tags | Array<string> | いいえ |  |
| sso | string | いいえ |  |
| isCrawler | boolean | いいえ |  |
| includeUserInfo | boolean | いいえ |  |

## レスポンス

返却: [`GetFeedPostsPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublicResponse.ts)

## 例

[inline-code-attrs-start title = 'getFeedPostsPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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