## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| options | GetTicketsOptions | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetTicketsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (ticketsOpt, httpResp) = client.getTickets(tenantId = "my-tenant-123", options = GetTicketsOptions())
if ticketsOpt.isSome:
  let tickets = ticketsOpt.get()
  # χρησιμοποιήστε τα εισιτήρια όπως χρειάζεται
[inline-code-end]

---