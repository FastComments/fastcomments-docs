## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| user_id | String | לא |  |

## תגובה

מחזיר: [`GetSubscriptionsApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_subscriptions_api_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_subscriptions'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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