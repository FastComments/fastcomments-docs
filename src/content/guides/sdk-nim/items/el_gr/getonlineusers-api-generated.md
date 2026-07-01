---
Παρόντες online θεατές μιας σελίδας: άτομα των οποίων η συνεδρία websocket είναι εγγεγραμμένη στη σελίδα αυτή τη στιγμή.  
Επιστρέφει anonCount + totalCount (συνδρομητές σε ολόκληρο το δωμάτιο, συμπεριλαμβανομένων των ανώνυχων θεατών που δεν απαριθμούμε).

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOnlineUsersOptions | No |  |

## Απάντηση

Επιστρέφει: [`Option[PageUsersOnlineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_online_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOnlineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetOnlineUsersOptions()
let (onlineUsersOpt, httpResp) = client.getOnlineUsers(tenantId = "my-tenant-123", urlId = "news/article-title", options = opts)
if onlineUsersOpt.isSome:
  let onlineUsers = onlineUsersOpt.get()
  echo onlineUsers
[inline-code-end]

---