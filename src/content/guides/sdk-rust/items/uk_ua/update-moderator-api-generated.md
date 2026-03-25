## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| update_moderator_body | models::UpdateModeratorBody | Так |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад update_moderator'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UpdateModeratorParams = UpdateModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-987".to_string(),
        update_moderator_body: models::UpdateModeratorBody {
            username: Some("jane.doe".to_string()),
            email: Some("jane.doe@acme.com".to_string()),
            role: Some("senior_moderator".to_string()),
            active: Some(true),
            notes: Some("Promoted after successful trial period".to_string()),
        },
    };
    let response: FlagCommentPublic200Response = update_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---