## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| comment_id | String | Tak |  |
| dir | i32 | Tak |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_success_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_comment_vote_user_names'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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