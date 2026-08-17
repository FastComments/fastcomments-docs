## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Да |  |
| spam | bool | Не |  |
| perm_not_spam | bool | Не |  |
| broadcast_id | String | Не |  |
| sso | String | Не |  |

## Одговор

Враћа: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Пример

[inline-code-attrs-start title = 'post_set_comment_spam_status Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PostSetCommentSpamStatusParams {
        tenant_id: "acme-corp-tenant".into(),
        comment_id: "comment-12345".into(),
        spam: Some(true),
        perm_not_spam: Some(false),
        broadcast_id: Some("broadcast-678".into()),
        sso: Some("user@example.com".into()),
    };
    post_set_comment_spam_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]