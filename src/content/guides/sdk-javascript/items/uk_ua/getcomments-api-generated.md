## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| page | number | Ні |  |
| limit | number | Ні |  |
| skip | number | Ні |  |
| asTree | boolean | Ні |  |
| skipChildren | number | Ні |  |
| limitChildren | number | Ні |  |
| maxTreeDepth | number | Ні |  |
| urlId | string | Ні |  |
| userId | string | Ні |  |
| anonUserId | string | Ні |  |
| contextUserId | string | Ні |  |
| hashTag | string | Ні |  |
| parentId | string | Ні |  |
| direction | SortDirections | Ні |  |

## Відповідь

Повертає: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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