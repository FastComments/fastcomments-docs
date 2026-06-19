Τρέχοντες συνδεδεμένοι θεατές μιας σελίδας: άτομα των οποίων η websocket συνεδρία είναι εγγεγραμμένη στη σελίδα αυτή αυτή τη στιγμή.
Επιστρέφει anonCount + totalCount (συνδρομητές σε όλο το δωμάτιο, συμπεριλαμβανομένων των ανώνυμων θεατών που δεν απαριθμούμε).

## Παράμετροι

| Name | Type | Required | Description |
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