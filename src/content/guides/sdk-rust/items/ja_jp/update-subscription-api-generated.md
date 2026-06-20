## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| update_api_user_subscription_data | models::UpdateApiUserSubscriptionData | はい |  |
| user_id | String | いいえ |  |

## レスポンス

戻り値: [`UpdateSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_subscription_api_response.rs)

## 例

[inline-code-attrs-start title = 'update_subscription の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<UpdateSubscriptionApiResponse, Error> {
    let params: UpdateSubscriptionParams = UpdateSubscriptionParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "sub-8f3a2b".to_string(),
        update_api_user_subscription_data: models::UpdateApiUserSubscriptionData {
            active: true,
            plan: "standard".to_string(),
            topics: vec!["news/article".to_string(), "product/updates".to_string()],
        },
        user_id: Some("user-987".to_string()),
    };
    let updated: UpdateSubscriptionApiResponse = update_subscription(&configuration, params).await?;
    Ok(updated)
}
[inline-code-end]

---