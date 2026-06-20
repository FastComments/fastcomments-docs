## Paramètres

| Name | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| page | i32 | Non |  |
| limit | i32 | Non |  |
| skip | i32 | Non |  |
| as_tree | bool | Non |  |
| skip_children | i32 | Non |  |
| limit_children | i32 | Non |  |
| max_tree_depth | i32 | Non |  |
| url_id | String | Non |  |
| user_id | String | Non |  |
| anon_user_id | String | Non |  |
| context_user_id | String | Non |  |
| hash_tag | String | Non |  |
| parent_id | String | Non |  |
| direction | models::SortDirections | Non |  |
| from_date | i64 | Non |  |
| to_date | i64 | Non |  |

## Réponse

Renvoie : [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_comments'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetCommentsParams = GetCommentsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1),
        limit: Some(25),
        skip: Some(0),
        as_tree: Some(true),
        skip_children: Some(0),
        limit_children: Some(5),
        max_tree_depth: Some(3),
        url_id: Some("news/article/2026/06/fast-rust".to_string()),
        user_id: Some("user-1234".to_string()),
        anon_user_id: Some("anon-5678".to_string()),
        context_user_id: Some("context-999".to_string()),
        hash_tag: Some("release".to_string()),
        parent_id: Some("comment-9876".to_string()),
        direction: Some(models::SortDirections::Desc),
        from_date: Some(1_689_000_000_i64),
        to_date: Some(1_689_086_400_i64),
    };

    let response: ApiGetCommentsResponse = get_comments(configuration, params).await?;
    Ok(())
}
[inline-code-end]