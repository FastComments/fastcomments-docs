## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | No |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Respuesta

Devuelve: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Crear identificador de tenant (parámetro opcional)
const tenantId: string = "tenant_9f8c2b7a";

// Preparar entradas individuales de etiquetas
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

// Cuerpo para creación masiva (parámetro opcional)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Llamar a la función async global y asignar el resultado tipado
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]

---