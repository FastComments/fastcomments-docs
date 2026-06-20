## Параметри

| Name | Type | Задължително | Описание |
|------|------|--------------|----------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| un_block_from_comment_params | models::UnBlockFromCommentParams | Да |  |
| user_id | String | Не |  |
| anon_user_id | String | Не |  |

## Отговор

Връща: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/unblock_success.rs)

## Пример

[inline-code-attrs-start title = 'Пример за un_block_user_from_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: UnBlockUserFromCommentParams = UnBlockUserFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/comments/42".to_string(),
        un_block_from_comment_params: models::UnBlockFromCommentParams {
            reason: Some("mistaken moderation".to_string()),
            unblock_children: Some(true),
        },
        user_id: Some("user-12345".to_string()),
        anon_user_id: None,
    };
    let success: UnblockSuccess = un_block_user_from_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---