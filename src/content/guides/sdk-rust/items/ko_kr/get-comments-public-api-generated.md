req
tenantId
urlId

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 예 |  |
| page | i32 | 아니요 |  |
| direction | models::SortDirections | 아니요 |  |
| sso | String | 아니요 |  |
| skip | i32 | 아니요 |  |
| skip_children | i32 | 아니요 |  |
| limit | i32 | 아니요 |  |
| limit_children | i32 | 아니요 |  |
| count_children | bool | 아니요 |  |
| fetch_page_for_comment_id | String | 아니요 |  |
| include_config | bool | 아니요 |  |
| count_all | bool | 아니요 |  |
| includei10n | bool | 아니요 |  |
| locale | String | 아니요 |  |
| modules | String | 아니요 |  |
| is_crawler | bool | 아니요 |  |
| include_notification_count | bool | 아니요 |  |
| as_tree | bool | 아니요 |  |
| max_tree_depth | i32 | 아니요 |  |
| use_full_translation_ids | bool | 아니요 |  |
| parent_id | String | 아니요 |  |
| search_text | String | 아니요 |  |
| hash_tags | Vec<String> | 아니요 |  |
| user_id | String | 아니요 |  |
| custom_config_str | String | 아니요 |  |
| after_comment_id | String | 아니요 |  |
| before_comment_id | String | 아니요 |  |

## 응답

반환: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_comments_public 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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