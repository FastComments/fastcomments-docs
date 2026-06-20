req
tenantId
urlId

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| page | i32 | Ne |  |
| direction | models::SortDirections | Ne |  |
| sso | String | Ne |  |
| skip | i32 | Ne |  |
| skip_children | i32 | Ne |  |
| limit | i32 | Ne |  |
| limit_children | i32 | Ne |  |
| count_children | bool | Ne |  |
| fetch_page_for_comment_id | String | Ne |  |
| include_config | bool | Ne |  |
| count_all | bool | Ne |  |
| includei10n | bool | Ne |  |
| locale | String | Ne |  |
| modules | String | Ne |  |
| is_crawler | bool | Ne |  |
| include_notification_count | bool | Ne |  |
| as_tree | bool | Ne |  |
| max_tree_depth | i32 | Ne |  |
| use_full_translation_ids | bool | Ne |  |
| parent_id | String | Ne |  |
| search_text | String | Ne |  |
| hash_tags | Vec<String> | Ne |  |
| user_id | String | Ne |  |
| custom_config_str | String | Ne |  |
| after_comment_id | String | Ne |  |
| before_comment_id | String | Ne |  |

## Odgovor

Vraća: `GetCommentsResponseWithPresencePublicComment`

## Primer

[inline-code-attrs-start title = 'get_comments_public Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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