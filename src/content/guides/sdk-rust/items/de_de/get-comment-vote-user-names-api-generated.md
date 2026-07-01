## Parameter

| Name      | Typ   | Erforderlich | Beschreibung |
|-----------|-------|--------------|--------------|
| tenant_id | String | Ja           |  |
| comment_id| String | Ja           |  |
| dir       | i32    | Ja           |  |
| sso       | String | Nein         |  |

## Antwort

Rückgabe: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_success_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_comment_vote_user_names Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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