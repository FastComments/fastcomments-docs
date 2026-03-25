## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ні |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Ні |  |

## Відповідь

Повертає: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Створіть ідентифікатор tenant (необов'язковий параметр)
const tenantId: string = "tenant_9f8c2b7a";

// Підготуйте окремі записи тегів
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

// Тіло для масового створення (необов'язковий параметр)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Викличте глобальну асинхронну функцію та присвойте типізований результат
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]