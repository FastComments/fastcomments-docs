## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenant_id | String | Да |  |
| page | i32 | Нет |  |
| limit | i32 | Нет |  |
| skip | i32 | Нет |  |
| as_tree | bool | Нет |  |
| skip_children | i32 | Нет |  |
| limit_children | i32 | Нет |  |
| max_tree_depth | i32 | Нет |  |
| url_id | String | Нет |  |
| user_id | String | Нет |  |
| anon_user_id | String | Нет |  |
| context_user_id | String | Нет |  |
| hash_tag | String | Нет |  |
| parent_id | String | Нет |  |
| direction | models::SortDirections | Нет |  |
| from_date | i64 | Нет |  |
| to_date | i64 | Нет |  |

## Ответ

Возвращает: [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_comments'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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