## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| comment_id | String | Tak |  |
| approved | bool | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_approved_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład post_set_comment_approval_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: PostSetCommentApprovalStatusParams = PostSetCommentApprovalStatusParams {
        comment_id: String::from("news/article/2026-06-19/post-42/comment-128"),
        approved: Some(true),
        sso: Some(String::from("sso:user:acme:eyJhbGciOiJIUzI1Ni")),
    };
    let response: SetCommentApprovedResponse = post_set_comment_approval_status(&configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---