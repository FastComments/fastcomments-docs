## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| skip | f64 | Ні |  |

## Відповідь

Повертає: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderators_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_moderators'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetModeratorsParams = GetModeratorsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let moderators: GetModerators200Response = get_moderators(&configuration, params).await?;
    let _moderators = moderators;
    Ok(())
}
[inline-code-end]

---