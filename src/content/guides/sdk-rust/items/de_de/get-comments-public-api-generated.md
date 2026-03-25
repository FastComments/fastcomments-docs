req
tenantId
urlId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| page | i32 | Nein |  |
| direction | models::SortDirections | Nein |  |
| sso | String | Nein |  |
| skip | i32 | Nein |  |
| skip_children | i32 | Nein |  |
| limit | i32 | Nein |  |
| limit_children | i32 | Nein |  |
| count_children | bool | Nein |  |
| fetch_page_for_comment_id | String | Nein |  |
| include_config | bool | Nein |  |
| count_all | bool | Nein |  |
| includei10n | bool | Nein |  |
| locale | String | Nein |  |
| modules | String | Nein |  |
| is_crawler | bool | Nein |  |
| include_notification_count | bool | Nein |  |
| as_tree | bool | Nein |  |
| max_tree_depth | i32 | Nein |  |
| use_full_translation_ids | bool | Nein |  |
| parent_id | String | Nein |  |
| search_text | String | Nein |  |
| hash_tags | Vec<String> | Nein |  |
| user_id | String | Nein |  |
| custom_config_str | String | Nein |  |
| after_comment_id | String | Nein |  |
| before_comment_id | String | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_public_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_comments_public Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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