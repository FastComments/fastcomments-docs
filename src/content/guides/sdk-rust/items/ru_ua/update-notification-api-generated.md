## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| update_notification_body | models::UpdateNotificationBody | Так |  |
| user_id | String | Ні |  |

## Відповідь

Повертає: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад update_notification'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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