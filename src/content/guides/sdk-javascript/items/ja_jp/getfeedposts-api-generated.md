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

## レスポンス

返却: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPosts200Response.ts)

## 例

[inline-code-attrs-start title = 'getFeedPosts の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_74321";
const afterId: string = "post_20250610_9b2f3";
const limit: number = 25;
const tags: Array<string> = ["product-updates", "announcements"];

const response: GetFeedPosts200Response = await getFeedPosts(tenantId, afterId, limit, tags);
[inline-code-end]

---