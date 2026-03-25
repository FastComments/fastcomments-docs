---

req
tenantId
urlId

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| page | i32 | Ні |  |
| direction | models::SortDirections | Ні |  |
| sso | String | Ні |  |
| skip | i32 | Ні |  |
| skip_children | i32 | Ні |  |
| limit | i32 | Ні |  |
| limit_children | i32 | Ні |  |
| count_children | bool | Ні |  |
| fetch_page_for_comment_id | String | Ні |  |
| include_config | bool | Ні |  |
| count_all | bool | Ні |  |
| includei10n | bool | Ні |  |
| locale | String | Ні |  |
| modules | String | Ні |  |
| is_crawler | bool | Ні |  |
| include_notification_count | bool | Ні |  |
| as_tree | bool | Ні |  |
| max_tree_depth | i32 | Ні |  |
| use_full_translation_ids | bool | Ні |  |
| parent_id | String | Ні |  |
| search_text | String | Ні |  |
| hash_tags | Vec<String> | Ні |  |
| user_id | String | Ні |  |
| custom_config_str | String | Ні |  |
| after_comment_id | String | Ні |  |
| before_comment_id | String | Ні |  |

## Відповідь

Повертає: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---