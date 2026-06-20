## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notification_count_response.rs)

## Primer

[inline-code-attrs-start title = 'get_user_notification_count Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetUserNotificationCountParams = GetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        sso: Some("user-42.sso.example".to_owned()),
    };
    let response: GetUserNotificationCountResponse = get_user_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---