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

戻り値: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsPublic200Response.ts)