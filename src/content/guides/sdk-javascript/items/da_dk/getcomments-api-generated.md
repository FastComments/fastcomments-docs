## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | number | Nej |  |
| limit | number | Nej |  |
| skip | number | Nej |  |
| asTree | boolean | Nej |  |
| skipChildren | number | Nej |  |
| limitChildren | number | Nej |  |
| maxTreeDepth | number | Nej |  |
| urlId | string | Nej |  |
| userId | string | Nej |  |
| anonUserId | string | Nej |  |
| contextUserId | string | Nej |  |
| hashTag | string | Nej |  |
| parentId | string | Nej |  |
| direction | SortDirections | Nej |  |

## Respons

Returnerer: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getComments Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // side
  20, // antal pr. side
  0, // spring over
  true, // som træstruktur
  1, // spring børn over
  3, // begræns antal børn
  4, // maksimal trædybde
  'articles/2026/new-product-launch', // url-id
  'user_7890', // bruger-id
  'anon_4f3b2', // anonym bruger-id
  undefined, // kontekst-bruger-id
  '#launch', // hashtag
  undefined // overordnet id
);
[inline-code-end]