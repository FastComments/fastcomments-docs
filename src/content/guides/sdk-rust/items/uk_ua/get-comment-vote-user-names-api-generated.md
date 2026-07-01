## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|------------|------|
| tenant_id | String | Так |  |
| comment_id | String | Так |  |
| dir | i32 | Так |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_success_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_comment_vote_user_names Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn demo(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetCommentVoteUserNamesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-123".to_string(),
        dir: 1,
        sso: Some("user-sso-id".to_string()),
    };
    let _response: GetCommentVoteUserNamesSuccessResponse =
        get_comment_vote_user_names(config, params).await?;
    Ok(())
}
[inline-code-end]