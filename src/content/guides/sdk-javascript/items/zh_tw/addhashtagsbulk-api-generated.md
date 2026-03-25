---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 否 |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | 否 |  |

## 回應

回傳：[`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## 範例

[inline-code-attrs-start title = 'addHashTagsBulk 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 建立租戶識別碼（可選參數）
const tenantId: string = "tenant_9f8c2b7a";

// 準備個別標籤項目
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

// 批次建立主體（可選參數）
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// 呼叫全域非同步函式並將回傳結果指定型別
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]

---