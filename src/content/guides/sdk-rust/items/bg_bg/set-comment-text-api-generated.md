## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Да |  |
| broadcast_id | String | Да |  |
| comment_text_update_request | models::CommentTextUpdateRequest | Да |  |
| edit_key | String | Не |  |
| sso | String | Не |  |

## Отговор

Връща: [`PublicApiSetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_set_comment_text_response.rs)

## Пример

[inline-code-attrs-start title = 'set_comment_text Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let comment_text_update_request = models::CommentTextUpdateRequest {
        text: "Edited comment text after moderation".to_string(),
        ..Default::default()
    };
    let params = SetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-98765".to_string(),
        broadcast_id: "news/article".to_string(),
        comment_text_update_request,
        edit_key: Some("edit-key-2024".to_string()),
        sso: Some("sso-token-789".to_string()),
    };
    let _response = set_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]