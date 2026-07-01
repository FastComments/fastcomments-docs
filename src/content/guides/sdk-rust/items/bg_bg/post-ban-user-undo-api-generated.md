## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| ban_user_undo_params | models::BanUserUndoParams | Yes |  |
| sso | String | No |  |

## Отговор

Връща: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'post_ban_user_undo Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PostBanUserUndoParams {
        tenant_id: "acme-corp-tenant".to_string(),
        ban_user_undo_params: models::BanUserUndoParams {
            user_id: "user-42".to_string(),
            note: Some("ban appeal accepted".to_string()),
        },
        sso: Some("sso-token-abc".to_string()),
    };
    let _ = post_ban_user_undo(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---