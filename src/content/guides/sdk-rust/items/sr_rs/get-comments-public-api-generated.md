req
tenantId
urlId

## Параметри

| Назив | Тип | Потребно | Опис |
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

## Одговор

Враћа: `GetCommentsResponseWithPresencePublicComment`

## Пример

[inline-code-attrs-start title = 'get_comments_public Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetCommentsPublicParams = GetCommentsPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    url_id: "news/article/climate-policy-2026".to_string(),
    page: Some(1),
    direction: Some(models::SortDirections::Desc),
    sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string()),
    skip: Some(0),
    skip_children: Some(0),
    limit: Some(25),
    limit_children: Some(5),
    count_children: Some(true),
    fetch_page_for_comment_id: Some("cmt-7890".to_string()),
    include_config: Some(true),
    count_all: Some(false),
    includei10n: Some(true),
    locale: Some("en-US".to_string()),
    modules: Some("reactions,tags".to_string()),
    is_crawler: Some(false),
    include_notification_count: Some(false),
    as_tree: Some(true),
    max_tree_depth: Some(3),
    use_full_translation_ids: Some(false),
    parent_id: None,
    search_text: Some("climate policy".to_string()),
    hash_tags: Some(vec!["environment".to_string(), "policy".to_string()]),
    user_id: Some("user-1234".to_string()),
    custom_config_str: None,
    after_comment_id: None,
    before_comment_id: None,
};
let response: GetCommentsResponseWithPresencePublicComment = get_comments_public(&configuration, params).await?;
[inline-code-end]