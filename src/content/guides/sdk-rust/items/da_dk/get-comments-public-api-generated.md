req
tenantId
urlId

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| page | i32 | Nej |  |
| direction | models::SortDirections | Nej |  |
| sso | String | Nej |  |
| skip | i32 | Nej |  |
| skip_children | i32 | Nej |  |
| limit | i32 | Nej |  |
| limit_children | i32 | Nej |  |
| count_children | bool | Nej |  |
| fetch_page_for_comment_id | String | Nej |  |
| include_config | bool | Nej |  |
| count_all | bool | Nej |  |
| includei10n | bool | Nej |  |
| locale | String | Nej |  |
| modules | String | Nej |  |
| is_crawler | bool | Nej |  |
| include_notification_count | bool | Nej |  |
| as_tree | bool | Nej |  |
| max_tree_depth | i32 | Nej |  |
| use_full_translation_ids | bool | Nej |  |
| parent_id | String | Nej |  |
| search_text | String | Nej |  |
| hash_tags | Vec<String> | Nej |  |
| user_id | String | Nej |  |
| custom_config_str | String | Nej |  |
| after_comment_id | String | Nej |  |
| before_comment_id | String | Nej |  |

## Svar

Returnerer: `GetCommentsResponseWithPresencePublicComment`

## Eksempel

[inline-code-attrs-start title = 'get_comments_public Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---