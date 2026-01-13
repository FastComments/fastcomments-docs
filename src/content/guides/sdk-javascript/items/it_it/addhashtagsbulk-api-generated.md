## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | No |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Risposta

Restituisce: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_3f2b9a';
  const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
    tags: [
      { name: 'performance', description: 'Comments about site performance', visibleToModeratorsOnly: false },
      { name: 'feature-request', description: 'Requests for new features', visibleToModeratorsOnly: true }
    ]
  };
  const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
  const resultWithNoTenant: AddHashTagsBulk200Response = await addHashTagsBulk(undefined, bulkCreateHashTagsBody);
  console.log(result, resultWithNoTenant);
})();
[inline-code-end]

---