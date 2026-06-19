---
Θεατές που είναι αυτήν τη στιγμή online σε μια σελίδα: άτομα των οποίων η websocket συνεδρία είναι αυτήν τη στιγμή εγγεγραμμένη στη σελίδα.
Επιστρέφει anonCount + totalCount (συνδρομητές στο δωμάτιο, συμπεριλαμβανομένων ανώνυμων θεατών που δεν απαριθμούμε).

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| afterName | string | Όχι |  |
| afterUserId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---