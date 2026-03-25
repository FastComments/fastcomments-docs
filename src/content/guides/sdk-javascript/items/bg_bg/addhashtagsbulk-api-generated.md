## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Не |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Не |  |

## Отговор

Връща: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Пример

[inline-code-attrs-start title = 'addHashTagsBulk Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Създайте идентификатор на наемателя (незадължителен параметър)
const tenantId: string = "tenant_9f8c2b7a";

// Подгответе отделни записи за таговете
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

// Тяло за масово създаване (незадължителен параметър)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Извикайте глобалната асинхронна функция и присвоете типизиран резултат
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]