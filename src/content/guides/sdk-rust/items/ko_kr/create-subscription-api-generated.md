## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| create_api_user_subscription_data | models::CreateApiUserSubscriptionData | 예 |  |

## 응답

반환: [`CreateSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_subscription_api_response.rs)

## 예시

[inline-code-attrs-start title = 'create_subscription 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<(), Error> {
    let subscription_data = models::CreateApiUserSubscriptionData {
        plan_id: "pro-plan".to_string(),
        trial_period_days: Some(14),
        start_date: Some("2024-01-01".to_string()),
        ..Default::default()
    };
    let params = CreateSubscriptionParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_api_user_subscription_data: subscription_data,
    };
    let _response: CreateSubscriptionApiResponse = create_subscription(configuration, params).await?;
    Ok(())
}
[inline-code-end]