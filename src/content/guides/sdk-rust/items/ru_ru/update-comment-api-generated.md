## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| updatable_comment_params | models::UpdatableCommentParams | Yes |  |
| context_user_id | String | No |  |
| do_spam_check | bool | No |  |
| is_live | bool | No |  |

## Ответ

Возвращает: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'update_comment Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<(), Error> {
    let updatable = models::UpdatableCommentParams {
        content: "Edited comment about the latest news article".to_string(),
        ..Default::default()
    };
    let params = UpdateCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-789".to_string(),
        updatable_comment_params: updatable,
        context_user_id: Some("reader-42".to_string()),
        do_spam_check: Some(true),
        is_live: Some(true),
    };
    let _ = update_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---