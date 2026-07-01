## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| block_from_comment_params | models::BlockFromCommentParams | Так |  |
| user_id | String | Ні |  |
| anon_user_id | String | Ні |  |

## Відповідь

Повертає: [`BlockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_success.rs)

## Приклад

[inline-code-attrs-start title = 'block_user_from_comment Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = BlockUserFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-9876".to_string(),
        block_from_comment_params: models::BlockFromCommentParams {
            reason: "spam".to_string(),
        },
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let _result: BlockSuccess = block_user_from_comment(configuration, params).await?;
    Ok(())
}
[inline-code-end]