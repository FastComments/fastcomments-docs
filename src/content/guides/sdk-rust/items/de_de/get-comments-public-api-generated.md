req
tenantId
urlId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| page | i32 | Nein |  |
| direction | models::SortDirections | Nein |  |
| sso | String | Nein |  |
| skip | i32 | Nein |  |
| skip_children | i32 | Nein |  |
| limit | i32 | Nein |  |
| limit_children | i32 | Nein |  |
| count_children | bool | Nein |  |
| fetch_page_for_comment_id | String | Nein |  |
| include_config | bool | Nein |  |
| count_all | bool | Nein |  |
| includei10n | bool | Nein |  |
| locale | String | Nein |  |
| modules | String | Nein |  |
| is_crawler | bool | Nein |  |
| include_notification_count | bool | Nein |  |
| as_tree | bool | Nein |  |
| max_tree_depth | i32 | Nein |  |
| use_full_translation_ids | bool | Nein |  |
| parent_id | String | Nein |  |
| search_text | String | Nein |  |
| hash_tags | Vec<String> | Nein |  |
| user_id | String | Nein |  |
| custom_config_str | String | Nein |  |
| after_comment_id | String | Nein |  |
| before_comment_id | String | Nein |  |

## Antwort

Gibt zurück: `GetCommentsResponseWithPresencePublicComment`

## Beispiel

[inline-code-attrs-start title = 'get_comments_public Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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