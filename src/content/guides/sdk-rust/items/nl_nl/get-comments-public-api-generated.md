req
tenantId
urlId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| page | i32 | Nee |  |
| direction | models::SortDirections | Nee |  |
| sso | String | Nee |  |
| skip | i32 | Nee |  |
| skip_children | i32 | Nee |  |
| limit | i32 | Nee |  |
| limit_children | i32 | Nee |  |
| count_children | bool | Nee |  |
| fetch_page_for_comment_id | String | Nee |  |
| include_config | bool | Nee |  |
| count_all | bool | Nee |  |
| includei10n | bool | Nee |  |
| locale | String | Nee |  |
| modules | String | Nee |  |
| is_crawler | bool | Nee |  |
| include_notification_count | bool | Nee |  |
| as_tree | bool | Nee |  |
| max_tree_depth | i32 | Nee |  |
| use_full_translation_ids | bool | Nee |  |
| parent_id | String | Nee |  |
| search_text | String | Nee |  |
| hash_tags | Vec<String> | Nee |  |
| user_id | String | Nee |  |
| custom_config_str | String | Nee |  |
| after_comment_id | String | Nee |  |
| before_comment_id | String | Nee |  |

## Response

Retourneert: `GetCommentsResponseWithPresencePublicComment`

## Example

[inline-code-attrs-start title = 'get_comments_public Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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