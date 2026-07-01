## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| user_id | String | Ні |  |

## Відповідь

Повертає: [`DeleteSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_subscription_api_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад delete_subscription'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteSubscriptionParams {
        tenant_id: "acme-corp".to_string(),
        id: "sub-2024-09".to_string(),
        user_id: Some("user-42".to_string()),
    };
    let _response = delete_subscription(&config, params).await?;
    Ok(())
}
[inline-code-end]