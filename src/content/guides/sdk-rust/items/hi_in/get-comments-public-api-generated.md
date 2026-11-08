req
tenantId
urlId

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| url_id | String | हाँ |  |
| page | i32 | नहीं |  |
| direction | models::SortDirections | नहीं |  |
| sso | String | नहीं |  |
| skip | i32 | नहीं |  |
| skip_children | i32 | नहीं |  |
| limit | i32 | नहीं |  |
| limit_children | i32 | नहीं |  |
| count_children | bool | नहीं |  |
| fetch_page_for_comment_id | String | नहीं |  |
| include_config | bool | नहीं |  |
| count_all | bool | नहीं |  |
| includei10n | bool | नहीं |  |
| locale | String | नहीं |  |
| modules | String | नहीं |  |
| is_crawler | bool | नहीं |  |
| include_notification_count | bool | नहीं |  |
| as_tree | bool | नहीं |  |
| max_tree_depth | i32 | नहीं |  |
| use_full_translation_ids | bool | नहीं |  |
| parent_id | String | नहीं |  |
| search_text | String | नहीं |  |
| hash_tags | Vec<String> | नहीं |  |
| user_id | String | नहीं |  |
| custom_config_str | String | नहीं |  |
| after_comment_id | String | नहीं |  |
| before_comment_id | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: `GetCommentsResponseWithPresencePublicComment`

## उदाहरण

[inline-code-attrs-start title = 'get_comments_public उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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