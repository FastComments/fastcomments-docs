req
tenantId
urlId

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| page | i32 | Ne |  |
| direction | models::SortDirections | Ne |  |
| sso | String | Ne |  |
| skip | i32 | Ne |  |
| skip_children | i32 | Ne |  |
| limit | i32 | Ne |  |
| limit_children | i32 | Ne |  |
| count_children | bool | Ne |  |
| fetch_page_for_comment_id | String | Ne |  |
| include_config | bool | Ne |  |
| count_all | bool | Ne |  |
| includei10n | bool | Ne |  |
| locale | String | Ne |  |
| modules | String | Ne |  |
| is_crawler | bool | Ne |  |
| include_notification_count | bool | Ne |  |
| as_tree | bool | Ne |  |
| max_tree_depth | i32 | Ne |  |
| use_full_translation_ids | bool | Ne |  |
| parent_id | String | Ne |  |
| search_text | String | Ne |  |
| hash_tags | Vec<String> | Ne |  |
| user_id | String | Ne |  |
| custom_config_str | String | Ne |  |
| after_comment_id | String | Ne |  |
| before_comment_id | String | Ne |  |

## Odgovor

Vraća: `GetCommentsResponseWithPresencePublicComment`

## Primjer

[inline-code-attrs-start title = 'Primjer get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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