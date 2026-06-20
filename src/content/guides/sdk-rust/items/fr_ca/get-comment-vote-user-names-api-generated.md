## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| dir | i32 | Oui |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_success_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_comment_vote_user_names'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_vote_names(configuration: &configuration::Configuration) -> Result<GetCommentVoteUserNamesSuccessResponse, Error> {
    let params: GetCommentVoteUserNamesParams = GetCommentVoteUserNamesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/2026/10/05/article-12345#comment-678".to_string(),
        dir: 1i32,
        sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.payload.signature".to_string()),
    };
    let response: GetCommentVoteUserNamesSuccessResponse = get_comment_vote_user_names(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---