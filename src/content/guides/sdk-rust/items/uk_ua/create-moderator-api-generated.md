## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenant_id | String | Так |  |
| create_moderator_body | models::CreateModeratorBody | Так |  |

## Відповідь

Повертає: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_moderator_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад create_moderator'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = CreateModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_moderator_body: models::CreateModeratorBody {
            email: "mod@example.com".to_string(),
            username: Some("mod_user".to_string()),
            permissions: vec!["delete".to_string(), "edit".to_string()],
            ..Default::default()
        },
    };
    let _response = create_moderator(configuration, params).await?;
    Ok(())
}
[inline-code-end]