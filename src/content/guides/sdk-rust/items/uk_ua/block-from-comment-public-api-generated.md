## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`BlockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_success.rs)

## Приклад

[inline-code-attrs-start title = 'block_from_comment_public Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = BlockFromCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-98765".to_string(),
        public_block_from_comment_params: models::PublicBlockFromCommentParams::default(),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _result: BlockSuccess = block_from_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---