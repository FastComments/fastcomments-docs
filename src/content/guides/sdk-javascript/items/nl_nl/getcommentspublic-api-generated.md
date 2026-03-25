req
tenantId
urlId

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nee |  |
| direction | SortDirections | Nee |  |
| sso | string | Nee |  |
| skip | number | Nee |  |
| skipChildren | number | Nee |  |
| limit | number | Nee |  |
| limitChildren | number | Nee |  |
| countChildren | boolean | Nee |  |
| fetchPageForCommentId | string | Nee |  |
| includeConfig | boolean | Nee |  |
| countAll | boolean | Nee |  |
| includei10n | boolean | Nee |  |
| locale | string | Nee |  |
| modules | string | Nee |  |
| isCrawler | boolean | Nee |  |
| includeNotificationCount | boolean | Nee |  |
| asTree | boolean | Nee |  |
| maxTreeDepth | number | Nee |  |
| useFullTranslationIds | boolean | Nee |  |
| parentId | string | Nee |  |
| searchText | string | Nee |  |
| hashTags | Array<string> | Nee |  |
| userId | string | Nee |  |
| customConfigStr | string | Nee |  |
| afterCommentId | string | Nee |  |
| beforeCommentId | string | Nee |  |

## Respons

Geeft terug: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsPublic Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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