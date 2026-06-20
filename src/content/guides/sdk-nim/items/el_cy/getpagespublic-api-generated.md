Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από την εφαρμογή επιφάνειας εργασίας του FChat για να συμπληρώσει τη λίστα δωματίων της.
Απαιτείται το `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη ρύθμιση για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| cursor | string | Όχι |  |
| limit | int | Όχι |  |
| q | string | Όχι |  |
| sortBy | PagesSortBy | Όχι |  |
| hasComments | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα χρήσης getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(
  tenantId = "my-tenant-123",
  cursor = "",
  limit = 0,
  q = "",
  sortBy = PagesSortBy(0),
  hasComments = false
)

if response.isSome:
  let pages = response.get()
  echo "Retrieved public pages: ", $pages
else:
  echo "No pages returned, HTTP status: ", $httpResponse.status
[inline-code-end]

---