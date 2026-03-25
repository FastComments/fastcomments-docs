req
tenantId
urlId

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| page | number | Hayır |  |
| direction | SortDirections | Hayır |  |
| sso | string | Hayır |  |
| skip | number | Hayır |  |
| skipChildren | number | Hayır |  |
| limit | number | Hayır |  |
| limitChildren | number | Hayır |  |
| countChildren | boolean | Hayır |  |
| fetchPageForCommentId | string | Hayır |  |
| includeConfig | boolean | Hayır |  |
| countAll | boolean | Hayır |  |
| includei10n | boolean | Hayır |  |
| locale | string | Hayır |  |
| modules | string | Hayır |  |
| isCrawler | boolean | Hayır |  |
| includeNotificationCount | boolean | Hayır |  |
| asTree | boolean | Hayır |  |
| maxTreeDepth | number | Hayır |  |
| useFullTranslationIds | boolean | Hayır |  |
| parentId | string | Hayır |  |
| searchText | string | Hayır |  |
| hashTags | Array<string> | Hayır |  |
| userId | string | Hayır |  |
| customConfigStr | string | Hayır |  |
| afterCommentId | string | Hayır |  |
| beforeCommentId | string | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getCommentsPublic Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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