Список страниц для арендатора. Используется клиентом FChat для настольных компьютеров для заполнения списка комнат.  
Требует, чтобы `enableFChat` был установлен в true в разрешённой пользовательской конфигурации для каждой страницы.  
Страницы, требующие SSO, фильтруются по доступу группы запрашивающего пользователя.

## Parameters

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| cursor | String | Нет |  |
| limit | i32 | Нет |  |
| q | String | Нет |  |
| sort_by | models::PagesSortBy | Нет |  |
| has_comments | bool | Нет |  |

## Response

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]