req
tenantId
urlId

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |
| page | i32 | Nie |  |
| direction | models::SortDirections | Nie |  |
| sso | String | Nie |  |
| skip | i32 | Nie |  |
| skip_children | i32 | Nie |  |
| limit | i32 | Nie |  |
| limit_children | i32 | Nie |  |
| count_children | bool | Nie |  |
| fetch_page_for_comment_id | String | Nie |  |
| include_config | bool | Nie |  |
| count_all | bool | Nie |  |
| includei10n | bool | Nie |  |
| locale | String | Nie |  |
| modules | String | Nie |  |
| is_crawler | bool | Nie |  |
| include_notification_count | bool | Nie |  |
| as_tree | bool | Nie |  |
| max_tree_depth | i32 | Nie |  |
| use_full_translation_ids | bool | Nie |  |
| parent_id | String | Nie |  |
| search_text | String | Nie |  |
| hash_tags | Vec<String> | Nie |  |
| user_id | String | Nie |  |
| custom_config_str | String | Nie |  |
| after_comment_id | String | Nie |  |
| before_comment_id | String | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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