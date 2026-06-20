---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| page | i32 | Nee |  |
| limit | i32 | Nee |  |
| skip | i32 | Nee |  |
| as_tree | bool | Nee |  |
| skip_children | i32 | Nee |  |
| limit_children | i32 | Nee |  |
| max_tree_depth | i32 | Nee |  |
| url_id | String | Nee |  |
| user_id | String | Nee |  |
| anon_user_id | String | Nee |  |
| context_user_id | String | Nee |  |
| hash_tag | String | Nee |  |
| parent_id | String | Nee |  |
| direction | models::SortDirections | Nee |  |
| from_date | i64 | Nee |  |
| to_date | i64 | Nee |  |

## Respons

Geeft terug: [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_comments Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---