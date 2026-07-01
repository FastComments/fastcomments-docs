## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| page | i32 | 아니오 |  |
| limit | i32 | 아니오 |  |
| skip | i32 | 아니오 |  |
| as_tree | bool | 아니오 |  |
| skip_children | i32 | 아니오 |  |
| limit_children | i32 | 아니오 |  |
| max_tree_depth | i32 | 아니오 |  |
| url_id | String | 아니오 |  |
| user_id | String | 아니오 |  |
| anon_user_id | String | 아니오 |  |
| context_user_id | String | 아니오 |  |
| hash_tag | String | 아니오 |  |
| parent_id | String | 아니오 |  |
| direction | models::SortDirections | 아니오 |  |
| from_date | i64 | 아니오 |  |
| to_date | i64 | 아니오 |  |

## 응답

반환: [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## 예시

[inline-code-attrs-start title = 'get_comments 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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