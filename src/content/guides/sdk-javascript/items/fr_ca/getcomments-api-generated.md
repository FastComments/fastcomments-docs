## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| page | number | Non |  |
| limit | number | Non |  |
| skip | number | Non |  |
| asTree | boolean | Non |  |
| skipChildren | number | Non |  |
| limitChildren | number | Non |  |
| maxTreeDepth | number | Non |  |
| urlId | string | Non |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |
| contextUserId | string | Non |  |
| hashTag | string | Non |  |
| parentId | string | Non |  |
| direction | SortDirections | Non |  |

## Réponse

Renvoie: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // page
  20, // limit
  0, // skip
  true, // asTree
  1, // skipChildren
  3, // limitChildren
  4, // maxTreeDepth
  'articles/2026/new-product-launch', // urlId
  'user_7890', // userId
  'anon_4f3b2', // anonUserId
  undefined, // contextUserId
  '#launch', // hashTag
  undefined // parentId
);
[inline-code-end]

---