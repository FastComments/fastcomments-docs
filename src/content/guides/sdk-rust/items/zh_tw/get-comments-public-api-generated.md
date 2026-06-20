req
tenantId
urlId

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 是 |  |
| page | i32 | 否 |  |
| direction | models::SortDirections | 否 |  |
| sso | String | 否 |  |
| skip | i32 | 否 |  |
| skip_children | i32 | 否 |  |
| limit | i32 | 否 |  |
| limit_children | i32 | 否 |  |
| count_children | bool | 否 |  |
| fetch_page_for_comment_id | String | 否 |  |
| include_config | bool | 否 |  |
| count_all | bool | 否 |  |
| includei10n | bool | 否 |  |
| locale | String | 否 |  |
| modules | String | 否 |  |
| is_crawler | bool | 否 |  |
| include_notification_count | bool | 否 |  |
| as_tree | bool | 否 |  |
| max_tree_depth | i32 | 否 |  |
| use_full_translation_ids | bool | 否 |  |
| parent_id | String | 否 |  |
| search_text | String | 否 |  |
| hash_tags | Vec<String> | 否 |  |
| user_id | String | 否 |  |
| custom_config_str | String | 否 |  |
| after_comment_id | String | 否 |  |
| before_comment_id | String | 否 |  |

## 回應

回傳: `GetCommentsResponseWithPresencePublicComment`

## 範例

[inline-code-attrs-start title = 'get_comments_public 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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