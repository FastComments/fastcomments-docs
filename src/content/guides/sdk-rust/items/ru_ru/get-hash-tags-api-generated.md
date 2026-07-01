---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenant_id | String | Да |  |
| page | f64 | Нет |  |

## Ответ

Возвращает: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_hash_tags_response.rs)

## Пример

[inline-code-attrs-start title = 'get_hash_tags Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetHashTagsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1.0),
    };
    let _response = get_hash_tags(&config, params).await?;
    Ok(())
}
[inline-code-end]

---