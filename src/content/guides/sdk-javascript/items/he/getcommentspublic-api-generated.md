req
tenantId
urlId

## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| page | number | לא |  |
| direction | SortDirections | לא |  |
| sso | string | לא |  |
| skip | number | לא |  |
| skipChildren | number | לא |  |
| limit | number | לא |  |
| limitChildren | number | לא |  |
| countChildren | boolean | לא |  |
| fetchPageForCommentId | string | לא |  |
| includeConfig | boolean | לא |  |
| countAll | boolean | לא |  |
| includei10n | boolean | לא |  |
| locale | string | לא |  |
| modules | string | לא |  |
| isCrawler | boolean | לא |  |
| includeNotificationCount | boolean | לא |  |
| asTree | boolean | לא |  |
| maxTreeDepth | number | לא |  |
| useFullTranslationIds | boolean | לא |  |
| parentId | string | לא |  |
| searchText | string | לא |  |
| hashTags | Array<string> | לא |  |
| userId | string | לא |  |
| customConfigStr | string | לא |  |
| afterCommentId | string | לא |  |
| beforeCommentId | string | לא |  |

## תגובה

מחזיר: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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