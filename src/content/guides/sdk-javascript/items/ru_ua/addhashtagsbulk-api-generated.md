## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Нет |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Нет |  |

## Ответ

Возвращает: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Создать идентификатор тенанта (необязательный параметр)
const tenantId: string = "tenant_9f8c2b7a";

// Подготовить отдельные записи тегов
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

// Тело для массового создания (необязательный параметр)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Вызвать глобальную асинхронную функцию и присвоить типизированный результат
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]

---