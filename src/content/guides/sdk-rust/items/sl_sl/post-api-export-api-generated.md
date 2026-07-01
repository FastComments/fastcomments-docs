## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| text_search | String | No |  |
| by_ip_from_comment | String | No |  |
| filters | String | No |  |
| search_filters | String | No |  |
| sorts | String | No |  |
| sso | String | No |  |

## Odgovor

Vrne: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_response.rs)

## Primer

[inline-code-attrs-start title = 'post_api_export Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn export_moderation() -> Result<(), Error> {
    let params = PostApiExportParams {
        tenant_id: "acme-corp-tenant".to_string(),
        text_search: Some("news/article".to_string()),
        by_ip_from_comment: Some("203.0.113.42".to_string()),
        filters: Some("status:pending".to_string()),
        search_filters: Some("created_at>2023-01-01".to_string()),
        sorts: Some("created_at_desc".to_string()),
        sso: None,
    };
    let _response = post_api_export(&configuration, params).await?;
    Ok(())
}
[inline-code-end]