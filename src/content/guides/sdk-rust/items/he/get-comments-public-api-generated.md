req
tenantId
urlId

## פרמטרים

| Name | Type | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| url_id | String | כן |  |
| page | i32 | לא |  |
| direction | models::SortDirections | לא |  |
| sso | String | לא |  |
| skip | i32 | לא |  |
| skip_children | i32 | לא |  |
| limit | i32 | לא |  |
| limit_children | i32 | לא |  |
| count_children | bool | לא |  |
| fetch_page_for_comment_id | String | לא |  |
| include_config | bool | לא |  |
| count_all | bool | לא |  |
| includei10n | bool | לא |  |
| locale | String | לא |  |
| modules | String | לא |  |
| is_crawler | bool | לא |  |
| include_notification_count | bool | לא |  |
| as_tree | bool | לא |  |
| max_tree_depth | i32 | לא |  |
| use_full_translation_ids | bool | לא |  |
| parent_id | String | לא |  |
| search_text | String | לא |  |
| hash_tags | Vec<String> | לא |  |
| user_id | String | לא |  |
| custom_config_str | String | לא |  |
| after_comment_id | String | לא |  |
| before_comment_id | String | לא |  |

## תגובה

מחזיר: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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