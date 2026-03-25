req
tenantId
urlId

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| page | number | Nie |  |
| direction | SortDirections | Nie |  |
| sso | string | Nie |  |
| skip | number | Nie |  |
| skipChildren | number | Nie |  |
| limit | number | Nie |  |
| limitChildren | number | Nie |  |
| countChildren | boolean | Nie |  |
| fetchPageForCommentId | string | Nie |  |
| includeConfig | boolean | Nie |  |
| countAll | boolean | Nie |  |
| includei10n | boolean | Nie |  |
| locale | string | Nie |  |
| modules | string | Nie |  |
| isCrawler | boolean | Nie |  |
| includeNotificationCount | boolean | Nie |  |
| asTree | boolean | Nie |  |
| maxTreeDepth | number | Nie |  |
| useFullTranslationIds | boolean | Nie |  |
| parentId | string | Nie |  |
| searchText | string | Nie |  |
| hashTags | Array<string> | Nie |  |
| userId | string | Nie |  |
| customConfigStr | string | Nie |  |
| afterCommentId | string | Nie |  |
| beforeCommentId | string | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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