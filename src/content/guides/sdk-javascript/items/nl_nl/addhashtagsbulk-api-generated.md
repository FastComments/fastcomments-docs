---
## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Nee |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Nee |  |

## Respons

Geeft terug: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'addHashTagsBulk Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_corp_987";
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [
    { name: "product-update", description: "Announcements about new product releases", visible: true },
    { name: "customer-support", description: "Customer support related discussions", visible: false }
  ],
  createdBy: "moderator_jane"
};
const resultWithTenant: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
const resultWithoutTenant: AddHashTagsBulk200Response = await addHashTagsBulk(undefined, bulkCreateHashTagsBody);
[inline-code-end]

---