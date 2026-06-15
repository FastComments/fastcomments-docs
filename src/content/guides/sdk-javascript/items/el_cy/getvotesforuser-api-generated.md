---
## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUser200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getVotesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9b8f7c6d';
const urlId: string = 'articles/product-update-2026';
const userId: string = 'user_c12345';
const anonUserId: string = 'anon_7f4e2a';
const votes: GetVotesForUser200Response = await getVotesForUser(tenantId, urlId, userId, anonUserId);
[inline-code-end]

---