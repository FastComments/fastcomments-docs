req
tenantId
urlId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nein |  |
| direction | SortDirections | Nein |  |
| sso | string | Nein |  |
| skip | number | Nein |  |
| skipChildren | number | Nein |  |
| limit | number | Nein |  |
| limitChildren | number | Nein |  |
| countChildren | boolean | Nein |  |
| fetchPageForCommentId | string | Nein |  |
| includeConfig | boolean | Nein |  |
| countAll | boolean | Nein |  |
| includei10n | boolean | Nein |  |
| locale | string | Nein |  |
| modules | string | Nein |  |
| isCrawler | boolean | Nein |  |
| includeNotificationCount | boolean | Nein |  |
| asTree | boolean | Nein |  |
| maxTreeDepth | number | Nein |  |
| useFullTranslationIds | boolean | Nein |  |
| parentId | string | Nein |  |
| searchText | string | Nein |  |
| hashTags | Array<string> | Nein |  |
| userId | string | Nein |  |
| customConfigStr | string | Nein |  |
| afterCommentId | string | Nein |  |
| beforeCommentId | string | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getCommentsPublic Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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