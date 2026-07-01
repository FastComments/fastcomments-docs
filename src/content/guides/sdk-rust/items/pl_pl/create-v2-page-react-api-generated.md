## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |
| id | String | Tak |  |
| title | String | Nie |  |

## Odpowiedź

Zwraca: `CreateV1PageReact`

## Przykład

[inline-code-attrs-start title = 'create_v2_page_react Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = CreateV2PageReactParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        id: "comment-123".to_string(),
        title: Some("Breaking News".to_string()),
    };
    let _react = create_v2_page_react(&configuration, params).await?;
    Ok(())
}
[inline-code-end]