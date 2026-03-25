## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| page | number | 否 |  |
| limit | number | 否 |  |
| skip | number | 否 |  |
| asTree | boolean | 否 |  |
| skipChildren | number | 否 |  |
| limitChildren | number | 否 |  |
| maxTreeDepth | number | 否 |  |
| urlId | string | 否 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |
| contextUserId | string | 否 |  |
| hashTag | string | 否 |  |
| parentId | string | 否 |  |
| direction | SortDirections | 否 |  |

## 回應

回傳: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## 範例

[inline-code-attrs-start title = 'getComments 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // 頁碼
  20, // 每頁數量
  0, // 跳過
  true, // 以樹狀回傳
  1, // 跳過子項
  3, // 子項限制
  4, // 最大樹層級
  'articles/2026/new-product-launch', // 網址識別碼
  'user_7890', // 使用者 ID
  'anon_4f3b2', // 匿名使用者 ID
  undefined, // 上下文使用者 ID
  '#launch', // 標籤
  undefined // 父項 ID
);
[inline-code-end]