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

Döndürür: `GetCommentsResponseWithPresencePublicComment`

## Örnek

[inline-code-attrs-start title = 'get_comments_public Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comments() -> Result<(), Error> {
    let params = GetCommentsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        page: Some(1),
        direction: Some(models::SortDirections::Desc),
        limit: Some(20),
        includei10n: Some(true),
        locale: Some("en-US".to_string()),
        as_tree: Some(true),
        max_tree_depth: Some(3),
        hash_tags: Some(vec!["rust".to_string(), "programming".to_string()]),
        ..Default::default()
    };
    let _response = get_comments_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]