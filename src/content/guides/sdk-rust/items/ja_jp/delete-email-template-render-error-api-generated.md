## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| error_id | String | Yes |  |

## レスポンス

戻り値: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'delete_email_template_render_error の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = DeleteEmailTemplateRenderErrorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "welcome-email".to_string(),
        error_id: "render-failure-123".to_string(),
    };
    let _ = delete_email_template_render_error(config, params).await?;
    Ok(())
}
[inline-code-end]