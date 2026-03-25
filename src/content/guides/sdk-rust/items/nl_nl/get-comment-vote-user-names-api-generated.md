## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| dir | i32 | Ja |  |
| sso | String | Nee |  |

## Antwoord

Retourneert: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_vote_user_names_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_comment_vote_user_names Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_vote_user_names() -> Result<GetCommentVoteUserNames200Response, Error> {
    let params: GetCommentVoteUserNamesParams = GetCommentVoteUserNamesParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("news/article-12345/comment-6789"),
        dir: 1,
        sso: Some(String::from("sso-token-01a2b3")),
    };
    let response: GetCommentVoteUserNames200Response =
        get_comment_vote_user_names(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---