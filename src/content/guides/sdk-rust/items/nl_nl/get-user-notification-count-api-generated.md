## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notification_count_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld get_user_notification_count'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let response = get_user_notification_count(&config, params).await?;
    println!("{:?}", response);
    Ok(())
}
[inline-code-end]