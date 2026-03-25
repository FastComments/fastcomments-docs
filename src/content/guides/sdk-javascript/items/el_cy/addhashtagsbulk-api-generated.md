## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Όχι |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Όχι |  |

## Απόκριση

Επιστρέφει: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Δημιουργία αναγνωριστικού tenant (προαιρετική παράμετρος)
const tenantId: string = "tenant_9f8c2b7a";

// Προετοιμασία μεμονωμένων εγγραφών ετικετών
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

// Σώμα για μαζική δημιουργία (προαιρετική παράμετρος)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Κλήση της παγκόσμιας ασύγχρονης συνάρτησης και ανάθεση του τυποποιημένου αποτελέσματος
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]

---