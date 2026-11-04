Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από το FChat desktop client για να συμπληρώσει τη λίστα των rooms του.
Απαιτεί το `enableFChat` να είναι true στο resolved custom config για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| cursor | String | Όχι |  |
| limit | i32 | Όχι |  |
| q | String | Όχι |  |
| sort_by | models::PagesSortBy | Όχι |  |
| has_comments | bool | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]