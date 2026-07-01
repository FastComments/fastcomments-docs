## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| redirect_url | String | No |  |

## 回應

返回: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 範例

[inline-code-attrs-start title = 'send_login_link 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = SendLoginLinkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        redirect_url: Some("https://acme.com/after-login".to_string()),
    };
    send_login_link(config, params).await?;
    Ok(())
}
[inline-code-end]

---