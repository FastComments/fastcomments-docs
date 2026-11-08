## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| page | i32 | नहीं |  |
| limit | i32 | नहीं |  |
| skip | i32 | नहीं |  |
| as_tree | bool | नहीं |  |
| skip_children | i32 | नहीं |  |
| limit_children | i32 | नहीं |  |
| max_tree_depth | i32 | नहीं |  |
| url_id | String | नहीं |  |
| user_id | String | नहीं |  |
| anon_user_id | String | नहीं |  |
| context_user_id | String | नहीं |  |
| hash_tag | String | नहीं |  |
| parent_id | String | नहीं |  |
| direction | models::SortDirections | नहीं |  |
| from_date | i64 | नहीं |  |
| to_date | i64 | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_comments उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comments() -> Result<(), Error> {
    let params = GetCommentsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1),
        limit: Some(20),
        skip: Some(0),
        as_tree: Some(true),
        skip_children: Some(5),
        limit_children: Some(10),
        max_tree_depth: Some(3),
        url_id: Some("news/article".to_string()),
        user_id: Some("user-123".to_string()),
        anon_user_id: Some("anon-456".to_string()),
        context_user_id: Some("ctx-789".to_string()),
        hash_tag: Some("rust".to_string()),
        parent_id: Some("parent-001".to_string()),
        direction: Some(models::SortDirections::Desc),
        from_date: Some(1_640_995_200),
        to_date: Some(1_641_081_600),
    };
    let _response = get_comments(&config, params).await?;
    Ok(())
}
[inline-code-end]