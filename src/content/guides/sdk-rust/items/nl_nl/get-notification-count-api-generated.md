## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| user_id | String | Nee |  |
| url_id | String | Nee |  |
| from_comment_id | String | Nee |  |
| viewed | bool | Nee |  |

## Respons

Retourneert: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_notification_count Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetNotificationCountParams = GetNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-67890".to_string()),
        url_id: Some("news/2026/03/25/election-updates".to_string()),
        from_comment_id: Some("cmt_42".to_string()),
        viewed: Some(false),
    };
    let response: GetNotificationCount200Response = get_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---