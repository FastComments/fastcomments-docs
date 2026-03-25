## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Nie |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Nie |  |

## Odpowiedź

Zwraca: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Utwórz identyfikator tenant (opcjonalny parametr)
const tenantId: string = "tenant_9f8c2b7a";

// Przygotuj pojedyncze wpisy tagów
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

// Ciało żądania tworzenia zbiorczego (opcjonalny parametr)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Wywołaj globalną funkcję asynchroniczną i przypisz wynik z określonym typem
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]