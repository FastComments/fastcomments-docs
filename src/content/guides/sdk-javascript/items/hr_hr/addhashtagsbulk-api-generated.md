## Parametri

| Name | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | No |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Odgovor

Vraća: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Kreirajte identifikator tenanta (neobavezni parametar)
const tenantId: string = "tenant_9f8c2b7a";

// Pripremite pojedinačne stavke oznaka
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

// Tijelo za masovno kreiranje (neobavezni parametar)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Pozovite globalnu asinhronu funkciju i dodijelite tipizirani rezultat
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]