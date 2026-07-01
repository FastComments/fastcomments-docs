## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| skip | f64 | Ні |  |

## Відповідь

Повертає: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderators_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_moderators Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetModeratorsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let _response = get_moderators(&configuration, params).await?;
    Ok(())
}
[inline-code-end]