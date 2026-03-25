## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| user_id | String | 아니오 |  |

## 응답

반환: [`GetSubscriptionsApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_subscriptions_api_response.rs)

## 예제

[inline-code-attrs-start title = 'get_subscriptions 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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