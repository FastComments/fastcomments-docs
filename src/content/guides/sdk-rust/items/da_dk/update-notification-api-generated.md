## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| update_notification_body | models::UpdateNotificationBody | Ja |  |
| user_id | String | Nej |  |

## Respons

Returnerer: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Eksempel

[inline-code-attrs-start title = 'update_notification Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let params = UpdateNotificationParams {
        tenant_id: "acme-corp".to_string(),
        id: "news/article".to_string(),
        update_notification_body: models::UpdateNotificationBody {
            title: "New article published".to_string(),
            content: "Read the latest updates in our blog.".to_string(),
        },
        user_id: Some("user-123".to_string()),
    };
    update_notification(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---