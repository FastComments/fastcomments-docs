verplicht
tenantId
urlId

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| page | i32 | Nee |  |
| direction | models::SortDirections | Nee |  |
| sso | String | Nee |  |
| skip | i32 | Nee |  |
| skip_children | i32 | Nee |  |
| limit | i32 | Nee |  |
| limit_children | i32 | Nee |  |
| count_children | bool | Nee |  |
| fetch_page_for_comment_id | String | Nee |  |
| include_config | bool | Nee |  |
| count_all | bool | Nee |  |
| includei10n | bool | Nee |  |
| locale | String | Nee |  |
| modules | String | Nee |  |
| is_crawler | bool | Nee |  |
| include_notification_count | bool | Nee |  |
| as_tree | bool | Nee |  |
| max_tree_depth | i32 | Nee |  |
| use_full_translation_ids | bool | Nee |  |
| parent_id | String | Nee |  |
| search_text | String | Nee |  |
| hash_tags | Vec<String> | Nee |  |
| user_id | String | Nee |  |
| custom_config_str | String | Nee |  |
| after_comment_id | String | Nee |  |
| before_comment_id | String | Nee |  |

## Antwoord

Retourneert: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_comments_public Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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