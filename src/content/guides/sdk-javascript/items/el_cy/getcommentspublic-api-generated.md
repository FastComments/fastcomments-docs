req
tenantId
urlId

## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| page | number | Όχι |  |
| direction | SortDirections | Όχι |  |
| sso | string | Όχι |  |
| skip | number | Όχι |  |
| skipChildren | number | Όχι |  |
| limit | number | Όχι |  |
| limitChildren | number | Όχι |  |
| countChildren | boolean | Όχι |  |
| fetchPageForCommentId | string | Όχι |  |
| includeConfig | boolean | Όχι |  |
| countAll | boolean | Όχι |  |
| includei10n | boolean | Όχι |  |
| locale | string | Όχι |  |
| modules | string | Όχι |  |
| isCrawler | boolean | Όχι |  |
| includeNotificationCount | boolean | Όχι |  |
| asTree | boolean | Όχι |  |
| maxTreeDepth | number | Όχι |  |
| useFullTranslationIds | boolean | Όχι |  |
| parentId | string | Όχι |  |
| searchText | string | Όχι |  |
| hashTags | Array<string> | Όχι |  |
| userId | string | Όχι |  |
| customConfigStr | string | Όχι |  |
| afterCommentId | string | Όχι |  |
| beforeCommentId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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