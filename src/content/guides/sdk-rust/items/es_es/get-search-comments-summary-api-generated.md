## Parameters

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| value | String | No |  |
| filters | String | No |  |
| search_filters | String | No |  |
| sso | String | No |  |

## Response

Devuelve: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_comment_search_response.rs)

## Example

[inline-code-attrs-start title = 'Ejemplo de get_search_comments_summary'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetSearchCommentsSummaryParams {
        tenant_id: "acme-corp-tenant".to_string(),
        value: Some("news/article".to_string()),
        filters: Some("status:approved".to_string()),
        search_filters: Some("author:john".to_string()),
        sso: Some("sso-token-123".to_string()),
    };
    let _response = get_search_comments_summary(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---