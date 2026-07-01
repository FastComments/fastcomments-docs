## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| page | i32 | Nein |  |
| limit | i32 | Nein |  |
| skip | i32 | Nein |  |
| as_tree | bool | Nein |  |
| skip_children | i32 | Nein |  |
| limit_children | i32 | Nein |  |
| max_tree_depth | i32 | Nein |  |
| url_id | String | Nein |  |
| user_id | String | Nein |  |
| anon_user_id | String | Nein |  |
| context_user_id | String | Nein |  |
| hash_tag | String | Nein |  |
| parent_id | String | Nein |  |
| direction | models::SortDirections | Nein |  |
| from_date | i64 | Nein |  |
| to_date | i64 | Nein |  |

## Antwort

Gibt zurück: [`ApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comments_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_comments Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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