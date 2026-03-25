## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | No |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Risposta

Restituisce: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Esempio

[inline-code-attrs-start title = 'addHashTagsBulk Esempio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Crea l'identificatore del tenant (parametro opzionale)
const tenantId: string = "tenant_9f8c2b7a";

// Prepara le singole voci dei tag
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

// Corpo per la creazione in blocco (parametro opzionale)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Chiama la funzione asincrona globale e assegna il risultato tipizzato
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]