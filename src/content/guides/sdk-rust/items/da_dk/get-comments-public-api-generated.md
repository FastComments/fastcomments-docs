req
tenantId
urlId

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| page | i32 | Nej |  |
| direction | models::SortDirections | Nej |  |
| sso | String | Nej |  |
| skip | i32 | Nej |  |
| skip_children | i32 | Nej |  |
| limit | i32 | Nej |  |
| limit_children | i32 | Nej |  |
| count_children | bool | Nej |  |
| fetch_page_for_comment_id | String | Nej |  |
| include_config | bool | Nej |  |
| count_all | bool | Nej |  |
| includei10n | bool | Nej |  |
| locale | String | Nej |  |
| modules | String | Nej |  |
| is_crawler | bool | Nej |  |
| include_notification_count | bool | Nej |  |
| as_tree | bool | Nej |  |
| max_tree_depth | i32 | Nej |  |
| use_full_translation_ids | bool | Nej |  |
| parent_id | String | Nej |  |
| search_text | String | Nej |  |
| hash_tags | Vec<String> | Nej |  |
| user_id | String | Nej |  |
| custom_config_str | String | Nej |  |
| after_comment_id | String | Nej |  |
| before_comment_id | String | Nej |  |

## Svar

Returnerer: `GetCommentsResponseWithPresencePublicComment`

## Eksempel

[inline-code-attrs-start title = 'get_comments_public Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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