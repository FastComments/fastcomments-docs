## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| user_id | String | 否 |  |

## 回應

返回：[`GetSubscriptionsApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_subscriptions_api_response.rs)

## 範例

[inline-code-attrs-start title = 'get_subscriptions 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_subscriptions(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetSubscriptionsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-12345".to_string()),
    };
    let _response: GetSubscriptionsApiResponse = get_subscriptions(config, params).await?;
    Ok(())
}
[inline-code-end]