## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| comment_id | String | Evet |  |
| set_comment_text_params | models::SetCommentTextParams | Evet |  |
| broadcast_id | String | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_response.rs)

## Örnek

[inline-code-attrs-start title = 'post_set_comment_text Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_comment(config: &configuration::Configuration) -> Result<(), Error> {
    let params = PostSetCommentTextParams {
        tenant_id: "acme-corp".to_string(),
        comment_id: "comment-9876".to_string(),
        set_comment_text_params: models::SetCommentTextParams {
            text: "Revised comment content".to_string(),
        },
        broadcast_id: Some("broadcast-2023".to_string()),
        sso: Some("sso-token-abc".to_string()),
    };
    let _response = post_set_comment_text(config, params).await?;
    Ok(())
}
[inline-code-end]