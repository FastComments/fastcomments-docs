## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| text_search | String | Ні |  |
| by_ip_from_comment | String | Ні |  |
| filters | String | Ні |  |
| search_filters | String | Ні |  |
| after_id | String | Ні |  |
| demo | bool | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`ModerationApiGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comment_ids_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_api_ids'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example() -> Result<ModerationApiGetCommentIdsResponse, Error> {
    let params: GetApiIdsParams = GetApiIdsParams {
        text_search: Some("climate policy debate".to_string()),
        by_ip_from_comment: Some("198.51.100.23".to_string()),
        filters: Some("status:approved,section:opinion".to_string()),
        search_filters: Some("author:guest".to_string()),
        after_id: Some("cmt_000123abc".to_string()),
        demo: Some(false),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let response: ModerationApiGetCommentIdsResponse = get_api_ids(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---