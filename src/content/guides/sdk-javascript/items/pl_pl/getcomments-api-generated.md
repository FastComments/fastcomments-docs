## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | number | No |  |
| limit | number | No |  |
| skip | number | No |  |
| asTree | boolean | No |  |
| skipChildren | number | No |  |
| limitChildren | number | No |  |
| maxTreeDepth | number | No |  |
| urlId | string | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |
| contextUserId | string | No |  |
| hashTag | string | No |  |
| parentId | string | No |  |
| direction | SortDirections | No |  |

## Odpowiedź

Zwraca: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // strona
  20, // limit
  0, // pomiń
  true, // jako drzewo
  1, // pomiń dzieci
  3, // limit dzieci
  4, // maks. głębokość drzewa
  'articles/2026/new-product-launch', // urlId
  'user_7890', // identyfikator użytkownika
  'anon_4f3b2', // identyfikator anonimowego użytkownika
  undefined, // identyfikator użytkownika kontekstu
  '#launch', // hashtag
  undefined // identyfikator rodzica
);
[inline-code-end]