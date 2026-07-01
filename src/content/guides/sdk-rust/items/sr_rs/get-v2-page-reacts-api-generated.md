## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |

## Odgovor

Vraća: `GetV2PageReacts`

## Primer

[inline-code-attrs-start title = 'get_v2_page_reacts Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetV2PageReactsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        page: Some(1),
        page_size: Some(50),
    };
    let _reacts = get_v2_page_reacts(&configuration, params).await?;
    Ok(())
}
[inline-code-end]