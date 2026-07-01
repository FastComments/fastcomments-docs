req
tenantId
urlId

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| page | i32 | Не |  |
| direction | models::SortDirections | Не |  |
| sso | String | Не |  |
| skip | i32 | Не |  |
| skip_children | i32 | Не |  |
| limit | i32 | Не |  |
| limit_children | i32 | Не |  |
| count_children | bool | Не |  |
| fetch_page_for_comment_id | String | Не |  |
| include_config | bool | Не |  |
| count_all | bool | Не |  |
| includei10n | bool | Не |  |
| locale | String | Не |  |
| modules | String | Не |  |
| is_crawler | bool | Не |  |
| include_notification_count | bool | Не |  |
| as_tree | bool | Не |  |
| max_tree_depth | i32 | Не |  |
| use_full_translation_ids | bool | Не |  |
| parent_id | String | Не |  |
| search_text | String | Не |  |
| hash_tags | Vec<String> | Не |  |
| user_id | String | Не |  |
| custom_config_str | String | Не |  |
| after_comment_id | String | Не |  |
| before_comment_id | String | Не |  |

## Отговор

Връща: `GetCommentsResponseWithPresencePublicComment`

## Пример

[inline-code-attrs-start title = 'Пример за get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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