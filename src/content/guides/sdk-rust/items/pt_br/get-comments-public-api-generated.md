req
tenantId
urlId

## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
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

Retorna: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetCommentsPublic200Response, Error> {
    let params: GetCommentsPublicParams = GetCommentsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/2026/03/25/top-story".to_string(),
        page: Some(1), direction: Some(models::SortDirections::Desc),
        sso: None, skip: Some(0), skip_children: Some(0),
        limit: Some(50), limit_children: Some(10), count_children: Some(true),
        fetch_page_for_comment_id: None, include_config: Some(true), count_all: Some(false),
        includei10n: Some(false), locale: Some("en-US".to_string()), modules: None,
        is_crawler: Some(false), include_notification_count: Some(false), as_tree: Some(true),
        max_tree_depth: Some(3), use_full_translation_ids: Some(false), parent_id: None,
        search_text: Some("climate policy debate".to_string()), hash_tags: Some(vec!["breaking".to_string(), "policy".to_string()]),
        user_id: None, custom_config_str: None, after_comment_id: None, before_comment_id: None,
    };
    let response: GetCommentsPublic200Response = get_comments_public(configuration, params).await?;
    Ok(response)
}
[inline-code-end]