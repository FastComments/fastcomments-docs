Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον desktop client του FChat για να συμπληρώσει τη λίστα δωματίων του.
Απαιτείται το `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη ρύθμιση για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται σύμφωνα με την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| cursor | string | Όχι |  |
| limit | number | Όχι |  |
| q | string | Όχι |  |
| sortBy | PagesSortBy | Όχι |  |
| hasComments | boolean | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]