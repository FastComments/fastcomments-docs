Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από το FChat desktop client για να συμπληρώσει τη λίστα των δωματίων του.
Απαιτεί το `enableFChat` να είναι true στο resolved custom config για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| options | GetPagesPublicOptions | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]