## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |

## Antwort

Gibt zurück: [`GetPagesApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_pages_api_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_pages Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetPagesParams = GetPagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        path: Some("news/article".to_string()),
        limit: Some(25),
        cursor: Some("cursor_01AZ".to_string()),
    };
    let pages: GetPagesApiResponse = get_pages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---