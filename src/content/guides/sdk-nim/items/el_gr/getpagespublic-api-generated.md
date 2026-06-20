Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον επιτραπέζιο πελάτη FChat για να γεμίσει τη λίστα δωματίων του. Απαιτείται το `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη ρύθμιση για κάθε σελίδα. Σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδων του αιτούμενου χρήστη.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
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

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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