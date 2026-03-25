## 参数

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

## 响应

返回: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## 示例

[inline-code-attrs-start title = 'getComments 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // 页码
  20, // 每页数量
  0, // 跳过
  true, // 是否以树形
  1, // 跳过子评论
  3, // 子评论限制
  4, // 最大树深度
  'articles/2026/new-product-launch', // URL 标识
  'user_7890', // 用户 ID
  'anon_4f3b2', // 匿名用户 ID
  undefined, // 上下文用户 ID
  '#launch', // 话题标签
  undefined // 父评论 ID
);
[inline-code-end]