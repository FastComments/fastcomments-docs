## Parameters

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| update_notification_body | models::UpdateNotificationBody | Da |  |
| user_id | String | Ne |  |

## Response

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Example

[inline-code-attrs-start title = 'Primjer update_notification'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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