req
tenantId
urlId

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| url_id | String | כן |  |
| page | i32 | לא |  |
| direction | models::SortDirections | לא |  |
| sso | String | לא |  |
| skip | i32 | לא |  |
| skip_children | i32 | לא |  |
| limit | i32 | לא |  |
| limit_children | i32 | לא |  |
| count_children | bool | לא |  |
| fetch_page_for_comment_id | String | לא |  |
| include_config | bool | לא |  |
| count_all | bool | לא |  |
| includei10n | bool | לא |  |
| locale | String | לא |  |
| modules | String | לא |  |
| is_crawler | bool | לא |  |
| include_notification_count | bool | לא |  |
| as_tree | bool | לא |  |
| max_tree_depth | i32 | לא |  |
| use_full_translation_ids | bool | לא |  |
| parent_id | String | לא |  |
| search_text | String | לא |  |
| hash_tags | Vec<String> | לא |  |
| user_id | String | לא |  |
| custom_config_str | String | לא |  |
| after_comment_id | String | לא |  |
| before_comment_id | String | לא |  |

## תגובה

מחזיר: `GetCommentsResponseWithPresencePublicComment`

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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