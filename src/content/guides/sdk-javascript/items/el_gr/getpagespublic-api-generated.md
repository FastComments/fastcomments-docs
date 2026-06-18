Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον desktop πελάτη FChat για να συμπληρώσει τη λίστα δωματίων του.
Απαιτείται το `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη ρύθμιση (custom config) για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Υποχρεωτικό | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| cursor | string | Όχι |  |
| limit | number | Όχι |  |
| q | string | Όχι |  |
| sortBy | PagesSortBy | Όχι |  |
| hasComments | boolean | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]

---