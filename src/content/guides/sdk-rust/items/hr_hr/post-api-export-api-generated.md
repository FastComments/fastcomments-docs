## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| text_search | String | Ne |  |
| by_ip_from_comment | String | Ne |  |
| filters | String | Ne |  |
| search_filters | String | Ne |  |
| sorts | String | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_response.rs)

## Primjer

[inline-code-attrs-start title = 'post_api_export Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---