req
tenantId
urlId

## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| url_id | String | Ναι |  |
| page | i32 | Όχι |  |
| direction | models::SortDirections | Όχι |  |
| sso | String | Όχι |  |
| skip | i32 | Όχι |  |
| skip_children | i32 | Όχι |  |
| limit | i32 | Όχι |  |
| limit_children | i32 | Όχι |  |
| count_children | bool | Όχι |  |
| fetch_page_for_comment_id | String | Όχι |  |
| include_config | bool | Όχι |  |
| count_all | bool | Όχι |  |
| includei10n | bool | Όχι |  |
| locale | String | Όχι |  |
| modules | String | Όχι |  |
| is_crawler | bool | Όχι |  |
| include_notification_count | bool | Όχι |  |
| as_tree | bool | Όχι |  |
| max_tree_depth | i32 | Όχι |  |
| use_full_translation_ids | bool | Όχι |  |
| parent_id | String | Όχι |  |
| search_text | String | Όχι |  |
| hash_tags | Vec<String> | Όχι |  |
| user_id | String | Όχι |  |
| custom_config_str | String | Όχι |  |
| after_comment_id | String | Όχι |  |
| before_comment_id | String | Όχι |  |

## Απόκριση

Επιστρέφει: `GetCommentsResponseWithPresencePublicComment`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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