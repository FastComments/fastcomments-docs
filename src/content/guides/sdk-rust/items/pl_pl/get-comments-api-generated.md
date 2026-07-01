## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| page | i32 | Nie |  |
| limit | i32 | Nie |  |
| skip | i32 | Nie |  |
| as_tree | bool | Nie |  |
| skip_children | i32 | Nie |  |
| limit_children | i32 | Nie |  |
| max_tree_depth | i32 | Nie |  |
| url_id | String | Nie |  |
| user_id | String | Nie |  |
| anon_user_id | String | Nie |  |
| context_user_id | String | Nie |  |
| hash_tag | String | Nie |  |
| parent_id | String | Nie |  |
| direction | models::SortDirections | Nie |  |
| from_date | i64 | Nie |  |
| to_date | i64 | Nie |  |

## Odpowiedź

Zwraca: [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_comments'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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