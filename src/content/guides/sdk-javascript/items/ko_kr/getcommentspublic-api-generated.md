---
req
tenantId
urlId

## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| page | number | 아니요 |  |
| direction | SortDirections | 아니요 |  |
| sso | string | 아니요 |  |
| skip | number | 아니요 |  |
| skipChildren | number | 아니요 |  |
| limit | number | 아니요 |  |
| limitChildren | number | 아니요 |  |
| countChildren | boolean | 아니요 |  |
| fetchPageForCommentId | string | 아니요 |  |
| includeConfig | boolean | 아니요 |  |
| countAll | boolean | 아니요 |  |
| includei10n | boolean | 아니요 |  |
| locale | string | 아니요 |  |
| modules | string | 아니요 |  |
| isCrawler | boolean | 아니요 |  |
| includeNotificationCount | boolean | 아니요 |  |
| asTree | boolean | 아니요 |  |
| maxTreeDepth | number | 아니요 |  |
| useFullTranslationIds | boolean | 아니요 |  |
| parentId | string | 아니요 |  |
| searchText | string | 아니요 |  |
| hashTags | Array<string> | 아니요 |  |
| userId | string | 아니요 |  |
| customConfigStr | string | 아니요 |  |
| afterCommentId | string | 아니요 |  |
| beforeCommentId | string | 아니요 |  |

## 응답

반환: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'getCommentsPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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