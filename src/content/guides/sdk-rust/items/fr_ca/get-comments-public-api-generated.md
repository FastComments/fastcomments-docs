req
tenantId
urlId

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |
| page | i32 | Non |  |
| direction | models::SortDirections | Non |  |
| sso | String | Non |  |
| skip | i32 | Non |  |
| skip_children | i32 | Non |  |
| limit | i32 | Non |  |
| limit_children | i32 | Non |  |
| count_children | bool | Non |  |
| fetch_page_for_comment_id | String | Non |  |
| include_config | bool | Non |  |
| count_all | bool | Non |  |
| includei10n | bool | Non |  |
| locale | String | Non |  |
| modules | String | Non |  |
| is_crawler | bool | Non |  |
| include_notification_count | bool | Non |  |
| as_tree | bool | Non |  |
| max_tree_depth | i32 | Non |  |
| use_full_translation_ids | bool | Non |  |
| parent_id | String | Non |  |
| search_text | String | Non |  |
| hash_tags | Vec<String> | Non |  |
| user_id | String | Non |  |
| custom_config_str | String | Non |  |
| after_comment_id | String | Non |  |
| before_comment_id | String | Non |  |

## Réponse

Retourne : `GetCommentsResponseWithPresencePublicComment`

## Exemple

[inline-code-attrs-start title = 'Exemple get_comments_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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