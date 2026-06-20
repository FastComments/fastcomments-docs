req
tenantId
urlId

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| page | i32 | Não |  |
| direction | models::SortDirections | Não |  |
| sso | String | Não |  |
| skip | i32 | Não |  |
| skip_children | i32 | Não |  |
| limit | i32 | Não |  |
| limit_children | i32 | Não |  |
| count_children | bool | Não |  |
| fetch_page_for_comment_id | String | Não |  |
| include_config | bool | Não |  |
| count_all | bool | Não |  |
| includei10n | bool | Não |  |
| locale | String | Não |  |
| modules | String | Não |  |
| is_crawler | bool | Não |  |
| include_notification_count | bool | Não |  |
| as_tree | bool | Não |  |
| max_tree_depth | i32 | Não |  |
| use_full_translation_ids | bool | Não |  |
| parent_id | String | Não |  |
| search_text | String | Não |  |
| hash_tags | Vec<String> | Não |  |
| user_id | String | Não |  |
| custom_config_str | String | Não |  |
| after_comment_id | String | Não |  |
| before_comment_id | String | Não |  |

## Resposta

Retorna: `GetCommentsResponseWithPresencePublicComment`

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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