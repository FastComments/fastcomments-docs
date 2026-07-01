## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| id | String | Ναι |  |
| update_notification_body | models::UpdateNotificationBody | Ναι |  |
| user_id | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα update_notification'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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