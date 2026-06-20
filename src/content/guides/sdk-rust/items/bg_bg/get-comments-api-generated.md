## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| page | i32 | Не |  |
| limit | i32 | Не |  |
| skip | i32 | Не |  |
| as_tree | bool | Не |  |
| skip_children | i32 | Не |  |
| limit_children | i32 | Не |  |
| max_tree_depth | i32 | Не |  |
| url_id | String | Не |  |
| user_id | String | Не |  |
| anon_user_id | String | Не |  |
| context_user_id | String | Не |  |
| hash_tag | String | Не |  |
| parent_id | String | Не |  |
| direction | models::SortDirections | Не |  |
| from_date | i64 | Не |  |
| to_date | i64 | Не |  |

## Отговор

Връща: [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_comments'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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