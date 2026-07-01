## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| redirect_url | String | いいえ |  |

## レスポンス

戻り値: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'send_login_link の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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