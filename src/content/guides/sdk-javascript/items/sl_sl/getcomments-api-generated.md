## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| page | number | Ne |  |
| limit | number | Ne |  |
| skip | number | Ne |  |
| asTree | boolean | Ne |  |
| skipChildren | number | Ne |  |
| limitChildren | number | Ne |  |
| maxTreeDepth | number | Ne |  |
| urlId | string | Ne |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |
| contextUserId | string | Ne |  |
| hashTag | string | Ne |  |
| parentId | string | Ne |  |
| direction | SortDirections | Ne |  |

## Odgovor

Vrača: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer uporabe getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // stran
  20, // omejitev
  0, // preskoči
  true, // kot drevo
  1, // preskoči podrejene
  3, // omeji podrejene
  4, // največja globina drevesa
  'articles/2026/new-product-launch', // urlId
  'user_7890', // userId
  'anon_4f3b2', // anonUserId
  undefined, // contextUserId
  '#launch', // hashTag
  undefined // parentId
);
[inline-code-end]