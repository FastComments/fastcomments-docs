## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| page | number | Hayır |  |
| limit | number | Hayır |  |
| skip | number | Hayır |  |
| asTree | boolean | Hayır |  |
| skipChildren | number | Hayır |  |
| limitChildren | number | Hayır |  |
| maxTreeDepth | number | Hayır |  |
| urlId | string | Hayır |  |
| userId | string | Hayır |  |
| anonUserId | string | Hayır |  |
| contextUserId | string | Hayır |  |
| hashTag | string | Hayır |  |
| parentId | string | Hayır |  |
| direction | SortDirections | Hayır |  |

## Yanıt

Döndürür: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getComments Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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