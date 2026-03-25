req
tenantId
urlId

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| page | i32 | Не |  |
| direction | models::SortDirections | Не |  |
| sso | String | Не |  |
| skip | i32 | Не |  |
| skip_children | i32 | Не |  |
| limit | i32 | Не |  |
| limit_children | i32 | Не |  |
| count_children | bool | Не |  |
| fetch_page_for_comment_id | String | Не |  |
| include_config | bool | Не |  |
| count_all | bool | Не |  |
| includei10n | bool | Не |  |
| locale | String | Не |  |
| modules | String | Не |  |
| is_crawler | bool | Не |  |
| include_notification_count | bool | Не |  |
| as_tree | bool | Не |  |
| max_tree_depth | i32 | Не |  |
| use_full_translation_ids | bool | Не |  |
| parent_id | String | Не |  |
| search_text | String | Не |  |
| hash_tags | Vec<String> | Не |  |
| user_id | String | Не |  |
| custom_config_str | String | Не |  |
| after_comment_id | String | Не |  |
| before_comment_id | String | Не |  |

## Одговор

Враћа: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

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