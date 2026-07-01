## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| page | f64 | Ne |  |
| count | f64 | Ne |  |
| text_search | String | Ne |  |
| by_ip_from_comment | String | Ne |  |
| filters | String | Ne |  |
| search_filters | String | Ne |  |
| sorts | String | Ne |  |
| demo | bool | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`ModerationApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comments_response.rs)

## Primer

[inline-code-attrs-start title = 'get_api_comments Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetApiCommentsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1.0),
        count: Some(20.0),
        text_search: Some("rust".to_string()),
        by_ip_from_comment: None,
        filters: Some("status:approved".to_string()),
        search_filters: Some("author:john".to_string()),
        sorts: Some("date:desc".to_string()),
        demo: Some(false),
        sso: None,
    };
    let _response = get_api_comments(&configuration, params).await?;
    Ok(())
}
[inline-code-end]