req
tenantId
urlId

## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| url_id | String | Sí |  |
| page | i32 | No |  |
| direction | models::SortDirections | No |  |
| sso | String | No |  |
| skip | i32 | No |  |
| skip_children | i32 | No |  |
| limit | i32 | No |  |
| limit_children | i32 | No |  |
| count_children | bool | No |  |
| fetch_page_for_comment_id | String | No |  |
| include_config | bool | No |  |
| count_all | bool | No |  |
| includei10n | bool | No |  |
| locale | String | No |  |
| modules | String | No |  |
| is_crawler | bool | No |  |
| include_notification_count | bool | No |  |
| as_tree | bool | No |  |
| max_tree_depth | i32 | No |  |
| use_full_translation_ids | bool | No |  |
| parent_id | String | No |  |
| search_text | String | No |  |
| hash_tags | Vec<String> | No |  |
| user_id | String | No |  |
| custom_config_str | String | No |  |
| after_comment_id | String | No |  |
| before_comment_id | String | No |  |

## Respuesta

Devuelve: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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