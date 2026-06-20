req
tenantId
urlId

## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|------|-------------|
| tenant_id | String | はい |  |
| url_id | String | はい |  |
| page | i32 | いいえ |  |
| direction | models::SortDirections | いいえ |  |
| sso | String | いいえ |  |
| skip | i32 | いいえ |  |
| skip_children | i32 | いいえ |  |
| limit | i32 | いいえ |  |
| limit_children | i32 | いいえ |  |
| count_children | bool | いいえ |  |
| fetch_page_for_comment_id | String | いいえ |  |
| include_config | bool | いいえ |  |
| count_all | bool | いいえ |  |
| includei10n | bool | いいえ |  |
| locale | String | いいえ |  |
| modules | String | いいえ |  |
| is_crawler | bool | いいえ |  |
| include_notification_count | bool | いいえ |  |
| as_tree | bool | いいえ |  |
| max_tree_depth | i32 | いいえ |  |
| use_full_translation_ids | bool | いいえ |  |
| parent_id | String | いいえ |  |
| search_text | String | いいえ |  |
| hash_tags | Vec<String> | いいえ |  |
| user_id | String | いいえ |  |
| custom_config_str | String | いいえ |  |
| after_comment_id | String | いいえ |  |
| before_comment_id | String | いいえ |  |

## レスポンス

戻り値: `GetCommentsResponseWithPresencePublicComment`

## 例

[inline-code-attrs-start title = 'get_comments_public の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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