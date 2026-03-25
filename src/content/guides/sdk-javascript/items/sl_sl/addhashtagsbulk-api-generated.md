## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenantId | string | Ne |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Ne |  |

## Odgovor

Vrne: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Ustvari identifikator najemnika (neobvezen parameter)
const tenantId: string = "tenant_9f8c2b7a";

// Pripravi posamezne vnose oznak
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

// Telo za množično ustvarjanje (neobvezen parameter)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Kliči globalno asinhrono funkcijo in dodeli tipiziran rezultat
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]