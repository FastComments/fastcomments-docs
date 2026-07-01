## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| page | i32 | Ne |  |
| limit | i32 | Ne |  |
| skip | i32 | Ne |  |
| as_tree | bool | Ne |  |
| skip_children | i32 | Ne |  |
| limit_children | i32 | Ne |  |
| max_tree_depth | i32 | Ne |  |
| url_id | String | Ne |  |
| user_id | String | Ne |  |
| anon_user_id | String | Ne |  |
| context_user_id | String | Ne |  |
| hash_tag | String | Ne |  |
| parent_id | String | Ne |  |
| direction | models::SortDirections | Ne |  |
| from_date | i64 | Ne |  |
| to_date | i64 | Ne |  |

## Odgovor

Vrne: [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_comments'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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