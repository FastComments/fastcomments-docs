## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| set_comment_text_params | models::SetCommentTextParams | Yes |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## 回應

Returns: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_response.rs)

## 範例

[inline-code-attrs-start title = 'post_set_comment_text 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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