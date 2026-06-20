## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| update_moderator_body | models::UpdateModeratorBody | Да |  |

## Ответ

Возвращает: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример update_moderator'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_moderator_example() -> Result<(), Error> {
    let params: UpdateModeratorParams = UpdateModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-1a2b3c".to_string(),
        update_moderator_body: models::UpdateModeratorBody {
            display_name: Some("Jane Doe".to_string()),
            email: Some("jane.doe@acme-corp.com".to_string()),
            role: Some("senior_moderator".to_string()),
            active: Some(true),
            permissions: Some(vec![
                "approve_comments".to_string(),
                "flag_spam".to_string(),
                "ban_users".to_string(),
            ]),
        },
    };
    let _empty: ApiEmptyResponse = update_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---