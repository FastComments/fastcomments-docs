## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| deleteComments | string | Όχι |  |
| commentDeleteMode | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4b7a9f2c";
const id: string = "user_9d3f1b6a";
const deleteComments: string = "true";
const commentDeleteMode: string = "permanent";
const result: APIEmptyResponse = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
[inline-code-end]

---