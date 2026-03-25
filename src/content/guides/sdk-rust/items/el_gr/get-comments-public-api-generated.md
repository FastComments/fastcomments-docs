req
tenantId
urlId

## Παράμετροι

| Όνομα | Τύπος | Απαιτούμενο | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| url_id | String | Ναι |  |
| page | i32 | Όχι |  |
| direction | models::SortDirections | Όχι |  |
| sso | String | Όχι |  |
| skip | i32 | Όχι |  |
| skip_children | i32 | Όχι |  |
| limit | i32 | Όχι |  |
| limit_children | i32 | Όχι |  |
| count_children | bool | Όχι |  |
| fetch_page_for_comment_id | String | Όχι |  |
| include_config | bool | Όχι |  |
| count_all | bool | Όχι |  |
| includei10n | bool | Όχι |  |
| locale | String | Όχι |  |
| modules | String | Όχι |  |
| is_crawler | bool | Όχι |  |
| include_notification_count | bool | Όχι |  |
| as_tree | bool | Όχι |  |
| max_tree_depth | i32 | Όχι |  |
| use_full_translation_ids | bool | Όχι |  |
| parent_id | String | Όχι |  |
| search_text | String | Όχι |  |
| hash_tags | Vec<String> | Όχι |  |
| user_id | String | Όχι |  |
| custom_config_str | String | Όχι |  |
| after_comment_id | String | Όχι |  |
| before_comment_id | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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