req
tenantId
urlId

## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nej |  |
| direction | SortDirections | Nej |  |
| sso | string | Nej |  |
| skip | number | Nej |  |
| skipChildren | number | Nej |  |
| limit | number | Nej |  |
| limitChildren | number | Nej |  |
| countChildren | boolean | Nej |  |
| fetchPageForCommentId | string | Nej |  |
| includeConfig | boolean | Nej |  |
| countAll | boolean | Nej |  |
| includei10n | boolean | Nej |  |
| locale | string | Nej |  |
| modules | string | Nej |  |
| isCrawler | boolean | Nej |  |
| includeNotificationCount | boolean | Nej |  |
| asTree | boolean | Nej |  |
| maxTreeDepth | number | Nej |  |
| useFullTranslationIds | boolean | Nej |  |
| parentId | string | Nej |  |
| searchText | string | Nej |  |
| hashTags | Array<string> | Nej |  |
| userId | string | Nej |  |
| customConfigStr | string | Nej |  |
| afterCommentId | string | Nej |  |
| beforeCommentId | string | Nej |  |

## Respons

Returnerer: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getCommentsPublic Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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