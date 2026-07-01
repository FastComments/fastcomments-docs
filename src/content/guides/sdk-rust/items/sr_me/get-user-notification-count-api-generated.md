## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| sso | String | No |  |

## Odgovor

Vraća: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notification_count_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_user_notification_count Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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