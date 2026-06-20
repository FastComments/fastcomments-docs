Seznam strani za najemnika. Uporablja se v namiznem odjemalcu FChat za napolnitev seznama sob.
Zahteva, da je `enableFChat` v razrešeni prilagojeni konfiguraciji za vsako stran nastavljen na true.
Strani, ki zahtevajo SSO, se filtrirajo glede na dostop skupine uporabnika, ki pošilja zahtevo.

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| cursor | String | No |  |
| limit | i32 | No |  |
| q | String | No |  |
| sort_by | models::PagesSortBy | No |  |
| has_comments | bool | No |  |

## Odgovor

Vrača: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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