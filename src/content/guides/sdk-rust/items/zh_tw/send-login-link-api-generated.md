---
## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| redirect_url | String | 否 |  |

## 回應

回傳: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 範例

[inline-code-attrs-start title = 'send_login_link 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn send_link_example() -> Result<(), Error> {
    let params: SendLoginLinkParams = SendLoginLinkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9876".to_string(),
        redirect_url: Some("https://acme.example.com/welcome".to_string()),
    };
    let response: ApiEmptyResponse = send_login_link(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---