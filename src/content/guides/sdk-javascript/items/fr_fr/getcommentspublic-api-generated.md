req
tenantId
urlId

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| page | number | Non |  |
| direction | SortDirections | Non |  |
| sso | string | Non |  |
| skip | number | Non |  |
| skipChildren | number | Non |  |
| limit | number | Non |  |
| limitChildren | number | Non |  |
| countChildren | boolean | Non |  |
| fetchPageForCommentId | string | Non |  |
| includeConfig | boolean | Non |  |
| countAll | boolean | Non |  |
| includei10n | boolean | Non |  |
| locale | string | Non |  |
| modules | string | Non |  |
| isCrawler | boolean | Non |  |
| includeNotificationCount | boolean | Non |  |
| asTree | boolean | Non |  |
| maxTreeDepth | number | Non |  |
| useFullTranslationIds | boolean | Non |  |
| parentId | string | Non |  |
| searchText | string | Non |  |
| hashTags | Array<string> | Non |  |
| userId | string | Non |  |
| customConfigStr | string | Non |  |
| afterCommentId | string | Non |  |
| beforeCommentId | string | Non |  |

## Réponse

Retourne : [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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