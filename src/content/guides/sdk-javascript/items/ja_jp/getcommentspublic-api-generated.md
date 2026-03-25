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

## 例

[inline-code-attrs-start title = 'getCommentsPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_eu-west_01';
const urlId: string = 'https://www.financialtimes.com/articles/2026/market-update-q1';
const response: GetCommentsPublic200Response = await getCommentsPublic(
  tenantId,
  urlId,
  2,
  undefined,
  'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.tokenPayload.signature',
  undefined,
  0,
  50,
  5,
  true,
  undefined,
  true,
  false,
  true,
  'en-US',
  'reactions,moderation',
  false,
  true,
  true,
  3,
  false,
  undefined,
  'performance',
  ['feature','fastcomments'],
  'user_9876',
  undefined,
  undefined,
  undefined
);
[inline-code-end]

---