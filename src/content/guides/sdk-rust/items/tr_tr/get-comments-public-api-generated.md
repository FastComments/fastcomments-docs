req
tenantId
urlId

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| url_id | String | Evet |  |
| page | i32 | Hayır |  |
| direction | models::SortDirections | Hayır |  |
| sso | String | Hayır |  |
| skip | i32 | Hayır |  |
| skip_children | i32 | Hayır |  |
| limit | i32 | Hayır |  |
| limit_children | i32 | Hayır |  |
| count_children | bool | Hayır |  |
| fetch_page_for_comment_id | String | Hayır |  |
| include_config | bool | Hayır |  |
| count_all | bool | Hayır |  |
| includei10n | bool | Hayır |  |
| locale | String | Hayır |  |
| modules | String | Hayır |  |
| is_crawler | bool | Hayır |  |
| include_notification_count | bool | Hayır |  |
| as_tree | bool | Hayır |  |
| max_tree_depth | i32 | Hayır |  |
| use_full_translation_ids | bool | Hayır |  |
| parent_id | String | Hayır |  |
| search_text | String | Hayır |  |
| hash_tags | Vec<String> | Hayır |  |
| user_id | String | Hayır |  |
| custom_config_str | String | Hayır |  |
| after_comment_id | String | Hayır |  |
| before_comment_id | String | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_comments_public Örnek'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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