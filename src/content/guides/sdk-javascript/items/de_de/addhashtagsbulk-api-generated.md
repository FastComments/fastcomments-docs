## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Nein |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Nein |  |

## Antwort

Gibt zurück: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'addHashTagsBulk Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Erstelle Mandantenkennung (optionaler Parameter)
const tenantId: string = "tenant_9f8c2b7a";

// Bereite einzelne Tag-Einträge vor
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

// Body für die Bulk-Erstellung (optionaler Parameter)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Rufe die globale asynchrone Funktion auf und weise das typisierte Ergebnis zu
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]

---