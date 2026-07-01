## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| set_comment_text_params | models::SetCommentTextParams | 是 |  |
| broadcast_id | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_response.rs)

## 示例

[inline-code-attrs-start title = 'post_set_comment_text 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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