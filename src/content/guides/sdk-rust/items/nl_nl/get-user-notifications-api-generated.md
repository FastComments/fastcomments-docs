## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenant_id | String | Ja |  |
| url_id | String | Nee |  |
| page_size | i32 | Nee |  |
| after_id | String | Nee |  |
| include_context | bool | Nee |  |
| after_created_at | i64 | Nee |  |
| unread_only | bool | Nee |  |
| dm_only | bool | Nee |  |
| no_dm | bool | Nee |  |
| include_translations | bool | Nee |  |
| include_tenant_notifications | bool | Nee |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_my_notifications_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld get_user_notifications'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetUserNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: Some("news/article".to_string()),
        page_size: Some(20),
        after_id: None,
        include_context: Some(true),
        after_created_at: None,
        unread_only: Some(false),
        dm_only: Some(false),
        no_dm: Some(true),
        include_translations: Some(false),
        include_tenant_notifications: Some(true),
        sso: None,
    };
    let _resp = get_user_notifications(&config, params).await?;
    Ok(())
}
[inline-code-end]

---