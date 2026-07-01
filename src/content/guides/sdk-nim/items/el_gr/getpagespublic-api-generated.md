List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

Λίστα σελίδων για έναν ενοικιαστή. Χρησιμοποιείται από τον πελάτη FChat στην επιφάνεια εργασίας για τη συμπλήρωση της λίστας δωματίων του.
Απαιτεί το `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη διαμόρφωση για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται βάσει της πρόσβασης ομάδας του χρήστη που κάνει την αίτηση.

## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetPagesPublicOptions | No |  |

## Response

Επιστρέφει: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]