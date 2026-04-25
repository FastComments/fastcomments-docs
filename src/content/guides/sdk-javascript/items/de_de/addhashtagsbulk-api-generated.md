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
const tenantId: string = 'tenant_acme_corp_01';
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [
    { name: 'feature-request', slug: 'feature-request', description: 'Requests for new capabilities', isActive: true, customConfig: { visibility: 'public' } as unknown as CustomConfigParameters }
  ]
};
const addHashTagsResponse: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);

const bulkCreateHashTagsBodyNoTenant: BulkCreateHashTagsBody = {
  tags: [
    { name: 'ux-feedback', slug: 'ux-feedback', description: 'User experience suggestions', isActive: true }
  ]
};
const addHashTagsResponseNoTenant: AddHashTagsBulk200Response = await addHashTagsBulk(undefined, bulkCreateHashTagsBodyNoTenant);
[inline-code-end]

---