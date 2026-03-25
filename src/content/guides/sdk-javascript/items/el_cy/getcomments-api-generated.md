## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| page | number | Όχι |  |
| limit | number | Όχι |  |
| skip | number | Όχι |  |
| asTree | boolean | Όχι |  |
| skipChildren | number | Όχι |  |
| limitChildren | number | Όχι |  |
| maxTreeDepth | number | Όχι |  |
| urlId | string | Όχι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |
| contextUserId | string | Όχι |  |
| hashTag | string | Όχι |  |
| parentId | string | Όχι |  |
| direction | SortDirections | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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