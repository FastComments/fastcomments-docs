## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Да |  |
| sso | String | Не |  |

## Отговор

Връща: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_banned_users_from_comment_response.rs)

## Пример

[inline-code-attrs-start title = 'get_ban_users_from_comment Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetBanUsersFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345".to_string(),
        sso: Some("sso-unique-id".to_string()),
    };
    let _response = get_ban_users_from_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---