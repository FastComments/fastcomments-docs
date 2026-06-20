List pages for a tenant. Koristi se od strane FChat desktop klijenta za popunjavanje njegove liste soba.
Zahteva da `enableFChat` bude true u razrešenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtevaju SSO filtriraju se u odnosu na pristupne grupe korisnika koji šalje zahtev.

## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| cursor | String | Ne |  |
| limit | i32 | Ne |  |
| q | String | Ne |  |
| sort_by | models::PagesSortBy | Ne |  |
| has_comments | bool | Ne |  |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Primer

[inline-code-attrs-start title = 'get_pages_public Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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