## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| user_id | String | 否 |  |

## 回應

回傳： [`GetSubscriptionsApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_subscriptions_api_response.rs)

## 範例

[inline-code-attrs-start title = 'get_subscriptions 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetSubscriptionsParams = GetSubscriptionsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-42@example.com".to_string()),
    };
    let subscriptions: GetSubscriptionsApiResponse = get_subscriptions(&configuration, params).await?;
    let _ = subscriptions;
    Ok(())
}
[inline-code-end]

---