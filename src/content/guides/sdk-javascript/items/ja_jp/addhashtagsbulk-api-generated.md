## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | いいえ |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | いいえ |  |

## Response

戻り値: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## 例

[inline-code-attrs-start title = 'addHashTagsBulk の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// テナント識別子を作成（オプションのパラメータ）
const tenantId: string = "tenant_9f8c2b7a";

// 個々のタグエントリを準備
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

// 一括作成ボディ（オプションのパラメータ）
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// グローバルな非同期関数を呼び出し、型付き結果を代入
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]

---