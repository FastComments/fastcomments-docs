## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| text_search | String | Hayır |  |
| by_ip_from_comment | String | Hayır |  |
| filters | String | Hayır |  |
| search_filters | String | Hayır |  |
| sorts | String | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_export_response.rs)

## Örnek

[inline-code-attrs-start title = 'post_api_export Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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