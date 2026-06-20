req
tenantId
urlId

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |
| page | i32 | Non |  |
| direction | models::SortDirections | Non |  |
| sso | String | Non |  |
| skip | i32 | Non |  |
| skip_children | i32 | Non |  |
| limit | i32 | Non |  |
| limit_children | i32 | Non |  |
| count_children | bool | Non |  |
| fetch_page_for_comment_id | String | Non |  |
| include_config | bool | Non |  |
| count_all | bool | Non |  |
| includei10n | bool | Non |  |
| locale | String | Non |  |
| modules | String | Non |  |
| is_crawler | bool | Non |  |
| include_notification_count | bool | Non |  |
| as_tree | bool | Non |  |
| max_tree_depth | i32 | Non |  |
| use_full_translation_ids | bool | Non |  |
| parent_id | String | Non |  |
| search_text | String | Non |  |
| hash_tags | Vec<String> | Non |  |
| user_id | String | Non |  |
| custom_config_str | String | Non |  |
| after_comment_id | String | Non |  |
| before_comment_id | String | Non |  |

## Réponse

Renvoie: `GetCommentsResponseWithPresencePublicComment`

## Exemple

[inline-code-attrs-start title = 'Exemple de get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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