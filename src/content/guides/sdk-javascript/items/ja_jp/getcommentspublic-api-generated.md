req
tenantId
urlId

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| page | number | いいえ |  |
| direction | SortDirections | いいえ |  |
| sso | string | いいえ |  |
| skip | number | いいえ |  |
| skipChildren | number | いいえ |  |
| limit | number | いいえ |  |
| limitChildren | number | いいえ |  |
| countChildren | boolean | いいえ |  |
| fetchPageForCommentId | string | いいえ |  |
| includeConfig | boolean | いいえ |  |
| countAll | boolean | いいえ |  |
| includei10n | boolean | いいえ |  |
| locale | string | いいえ |  |
| modules | string | いいえ |  |
| isCrawler | boolean | いいえ |  |
| includeNotificationCount | boolean | いいえ |  |
| asTree | boolean | いいえ |  |
| maxTreeDepth | number | いいえ |  |
| useFullTranslationIds | boolean | いいえ |  |
| parentId | string | いいえ |  |
| searchText | string | いいえ |  |
| hashTags | Array<string> | いいえ |  |
| userId | string | いいえ |  |
| customConfigStr | string | いいえ |  |
| afterCommentId | string | いいえ |  |
| beforeCommentId | string | いいえ |  |

## レスポンス

戻り値: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)