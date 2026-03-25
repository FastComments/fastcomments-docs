---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |

## Ответ

Возвращает: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderator_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_moderator'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-4521".to_string(),
        include_permissions: Some(true),
    };
    let moderator: GetModerator200Response = get_moderator(&configuration, params).await?;
    println!("{:#?}", moderator);
    Ok(())
}
[inline-code-end]

---