## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| update_api_user_subscription_data | models::UpdateApiUserSubscriptionData | 예 |  |
| user_id | String | 아니요 |  |

## 응답

반환: [`UpdateSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_subscription_api_response.rs)

## 예제

[inline-code-attrs-start title = 'update_subscription 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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