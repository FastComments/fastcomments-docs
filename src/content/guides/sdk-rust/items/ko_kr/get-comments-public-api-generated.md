req
tenantId
urlId

## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| page | i32 | 아니오 |  |
| direction | models::SortDirections | 아닙오 |  |
| sso | String | 아니오 |  |
| skip | i32 | 아니오 |  |
| skip_children | i32 | 아니오 |  |
| limit | i32 | 아니오 |  |
| limit_children | i32 | 아니오 |  |
| count_children | bool | 아니오 |  |
| fetch_page_for_comment_id | String | 아니오 |  |
| include_config | bool | 아니오 |  |
| count_all | bool | 아니오 |  |
| includei10n | bool | 아니오 |  |
| locale | String | 아니오 |  |
| modules | String | 아니오 |  |
| is_crawler | bool | 아니오 |  |
| include_notification_count | bool | 아니오 |  |
| as_tree | bool | 아니오 |  |
| max_tree_depth | i32 | 아니오 |  |
| use_full_translation_ids | bool | 아니오 |  |
| parent_id | String | 아니오 |  |
| search_text | String | 아니오 |  |
| hash_tags | Vec<String> | 아니오 |  |
| user_id | String | 아니오 |  |
| custom_config_str | String | 아니오 |  |
| after_comment_id | String | 아니오 |  |
| before_comment_id | String | 아니오 |  |

## 응답

반환: `GetCommentsResponseWithPresencePublicComment`

## 예제

[inline-code-attrs-start title = 'get_comments_public 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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