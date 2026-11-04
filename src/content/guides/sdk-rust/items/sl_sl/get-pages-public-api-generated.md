Izpiše strani za najemnika (tenant). Uporablja namizni odjemalec FChat za izpolnitev svojega seznama sob.
Zahteva, da je `enableFChat` nastavljen na true v razrešeni prilagojeni konfiguraciji za vsako stran.
Strani, ki zahtevajo SSO, so filtrirane glede na dostop do skupin zahtevajočega uporabnika.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| cursor | String | Ne |  |
| limit | i32 | Ne |  |
| q | String | Ne |  |
| sort_by | models::PagesSortBy | Ne |  |
| has_comments | bool | Ne |  |

## Odziv

Vrne: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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