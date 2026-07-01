## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_id | String | Да |  |
| dir | i32 | Да |  |
| sso | String | Не |  |

## Отговор

Връща: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_success_response.rs)

## Пример

[inline-code-attrs-start title = 'get_comment_vote_user_names Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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