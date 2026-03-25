## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 否 |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | 否 |  |

## 响应

返回：[`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## 示例

[inline-code-attrs-start title = 'addHashTagsBulk 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 创建租户标识符（可选参数）
const tenantId: string = "tenant_9f8c2b7a";

// 准备单个标签条目
const tag1: BulkCreateHashTagsBodyTagsInner = {
  name: "product-feedback",
  label: "Product Feedback",
  color: "#1f8a70",
  description: "User suggestions and enhancement requests",
  isActive: true
};

const tag2: BulkCreateHashTagsBodyTagsInner = {
  name: "bug-report",
  label: "Bug Report",
  color: "#d64545",
  description: "User-reported defects and issues",
  isActive: true
};

// 批量创建主体（可选参数）
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// 调用全局异步函数并将结果赋予类型化变量
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]