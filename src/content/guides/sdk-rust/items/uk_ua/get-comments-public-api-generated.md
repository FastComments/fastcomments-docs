req
tenantId
urlId

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| page | i32 | Ні |  |
| direction | models::SortDirections | Ні |  |
| sso | String | Ні |  |
| skip | i32 | Ні |  |
| skip_children | i32 | Ні |  |
| limit | i32 | Ні |  |
| limit_children | i32 | Ні |  |
| count_children | bool | Ні |  |
| fetch_page_for_comment_id | String | Ні |  |
| include_config | bool | Ні |  |
| count_all | bool | Ні |  |
| includei10n | bool | Ні |  |
| locale | String | Ні |  |
| modules | String | Ні |  |
| is_crawler | bool | Ні |  |
| include_notification_count | bool | Ні |  |
| as_tree | bool | Ні |  |
| max_tree_depth | i32 | Ні |  |
| use_full_translation_ids | bool | Ні |  |
| parent_id | String | Ні |  |
| search_text | String | Ні |  |
| hash_tags | Vec<String> | Ні |  |
| user_id | String | Ні |  |
| custom_config_str | String | Ні |  |
| after_comment_id | String | Ні |  |
| before_comment_id | String | Ні |  |

## Відповідь

Повертає: `GetCommentsResponseWithPresencePublicComment`

## Приклад

[inline-code-attrs-start title = 'Приклад get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---