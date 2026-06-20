## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |

## Odpowiedź

Zwraca: `GetV2PageReacts`

## Przykład

[inline-code-attrs-start title = 'get_v2_page_reacts Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_reacts_example() -> Result<(), Error> {
    let params: GetV2PageReactsParams = GetV2PageReactsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/rust-async-await".to_string(),
        include_counts: Some(true),
        limit: Some(50),
        cursor: Some("cursor_abc123".to_string()),
    };
    let reacts: GetV2PageReacts = get_v2_page_reacts(&configuration, params).await?;
    let _ = reacts;
    Ok(())
}
[inline-code-end]

---