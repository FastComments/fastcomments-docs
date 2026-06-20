Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare la lista delle stanze.
Richiede `enableFChat` to be true on the resolved custom config for each page.
Le pagine che richiedono SSO vengono filtrate in base ai gruppi di accesso dell'utente che effettua la richiesta.

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| cursor | String | No |  |
| limit | i32 | No |  |
| q | String | No |  |
| sort_by | models::PagesSortBy | No |  |
| has_comments | bool | No |  |

## Risposta

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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