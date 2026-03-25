req
tenantId
urlId

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| page | i32 | Нет |  |
| direction | models::SortDirections | Нет |  |
| sso | String | Нет |  |
| skip | i32 | Нет |  |
| skip_children | i32 | Нет |  |
| limit | i32 | Нет |  |
| limit_children | i32 | Нет |  |
| count_children | bool | Нет |  |
| fetch_page_for_comment_id | String | Нет |  |
| include_config | bool | Нет |  |
| count_all | bool | Нет |  |
| includei10n | bool | Нет |  |
| locale | String | Нет |  |
| modules | String | Нет |  |
| is_crawler | bool | Нет |  |
| include_notification_count | bool | Нет |  |
| as_tree | bool | Нет |  |
| max_tree_depth | i32 | Нет |  |
| use_full_translation_ids | bool | Нет |  |
| parent_id | String | Нет |  |
| search_text | String | Нет |  |
| hash_tags | Vec<String> | Нет |  |
| user_id | String | Нет |  |
| custom_config_str | String | Нет |  |
| after_comment_id | String | Нет |  |
| before_comment_id | String | Нет |  |

## Ответ

Возвращает: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## Пример

[inline-code-attrs-start title = 'get_comments_public Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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