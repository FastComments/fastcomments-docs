## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | No |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Одговор

Враћа: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Пример

[inline-code-attrs-start title = 'addHashTagsBulk Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Креирање идентификатора тенанта (опционални параметар)
const tenantId: string = "tenant_9f8c2b7a";

// Припремите појединачне уносе тагова
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

// Тело за масовно креирање (опционални параметар)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Позовите глобалну async функцију и доделите типизован резултат
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]