req
tenantId
urlId

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| page | i32 | No |  |
| direction | models::SortDirections | No |  |
| sso | String | No |  |
| skip | i32 | No |  |
| skip_children | i32 | No |  |
| limit | i32 | No |  |
| limit_children | i32 | No |  |
| count_children | bool | No |  |
| fetch_page_for_comment_id | String | No |  |
| include_config | bool | No |  |
| count_all | bool | No |  |
| includei10n | bool | No |  |
| locale | String | No |  |
| modules | String | No |  |
| is_crawler | bool | No |  |
| include_notification_count | bool | No |  |
| as_tree | bool | No |  |
| max_tree_depth | i32 | No |  |
| use_full_translation_ids | bool | No |  |
| parent_id | String | No |  |
| search_text | String | No |  |
| hash_tags | Vec<String> | No |  |
| user_id | String | No |  |
| custom_config_str | String | No |  |
| after_comment_id | String | No |  |
| before_comment_id | String | No |  |

## Ответ

Возвращает: `GetCommentsResponseWithPresencePublicComment`

## Пример

[inline-code-attrs-start title = 'get_comments_public Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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