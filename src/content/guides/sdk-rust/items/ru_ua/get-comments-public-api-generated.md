запрос
tenantId
urlId

## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| page | i32 | Нет |  |
| direction | models::SortDirections | Нет |  |
| sso | String | Нет |  |
| skip | i32 | Нет |  |
| skip_children | i32 | Нет |  |
| limit | i32 | Нет |  |
| limit_children | i32 | Нет |  |
| count_children | bool | Нет |  |
| fetch_page_for_comment_id | String | Нет |  |
| include_config | bool | Нет |  |
| count_all | bool | Нет |  |
| includei10n | bool | Нет |  |
| locale | String | Нет |  |
| modules | String | Нет |  |
| is_crawler | bool | Нет |  |
| include_notification_count | bool | Нет |  |
| as_tree | bool | Нет |  |
| max_tree_depth | i32 | Нет |  |
| use_full_translation_ids | bool | Нет |  |
| parent_id | String | Нет |  |
| search_text | String | Нет |  |
| hash_tags | Vec<String> | Нет |  |
| user_id | String | Нет |  |
| custom_config_str | String | Нет |  |
| after_comment_id | String | Нет |  |
| before_comment_id | String | Нет |  |

## Ответ

Возвращает: `GetCommentsResponseWithPresencePublicComment`

## Пример

[inline-code-attrs-start title = 'Пример get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comments() -> Result<(), Error> {
    let params = GetCommentsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        page: Some(1),
        direction: Some(models::SortDirections::Desc),
        limit: Some(20),
        includei10n: Some(true),
        locale: Some("en-US".to_string()),
        as_tree: Some(true),
        max_tree_depth: Some(3),
        hash_tags: Some(vec!["rust".to_string(), "programming".to_string()]),
        ..Default::default()
    };
    let _response = get_comments_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]