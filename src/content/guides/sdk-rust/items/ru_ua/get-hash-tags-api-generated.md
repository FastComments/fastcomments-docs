## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| page | f64 | Нет |  |

## Ответ

Возвращает: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_hash_tags_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_hash_tags'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_hash_tags() -> Result<GetHashTagsResponse, Error> {
    let params: GetHashTagsParams = GetHashTagsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(2.0),
    };
    let response: GetHashTagsResponse = get_hash_tags(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---