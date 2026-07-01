## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| broadcast_id | String | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primer

[inline-code-attrs-start title = 'post_un_flag_comment Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn unflag_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PostUnFlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        broadcast_id: Some("broadcast-987".to_string()),
        sso: Some("user@example.com".to_string()),
    };
    let _ = post_un_flag_comment(configuration, params).await?;
    Ok(())
}
[inline-code-end]