Επιστρέφει λίστα με σελίδες για έναν tenant. Χρησιμοποιείται από τον επιτραπέζιο πελάτη FChat για να συμπληρώσει τη λίστα δωμάτων του. Απαιτείται να είναι true το `enableFChat` στην επιλυμένη προσαρμοσμένη ρύθμιση (custom config) για κάθε σελίδα. Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| cursor | String | Όχι |  |
| limit | i32 | Όχι |  |
| q | String | Όχι |  |
| sort_by | models::PagesSortBy | Όχι |  |
| has_comments | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPagesPublicParams = GetPagesPublicParams {
    tenant_id: String::from("acme-corp-tenant"),
    cursor: Some(String::from("cursor_eyJwZl9pZCI6IjEyMyJ9")),
    limit: Some(50),
    q: Some(String::from("tag:release status:published")),
    sort_by: Some(models::PagesSortBy::CreatedAt),
    has_comments: Some(true),
};
let response: GetPublicPagesResponse = get_pages_public(&configuration, params).await?;
[inline-code-end]

---