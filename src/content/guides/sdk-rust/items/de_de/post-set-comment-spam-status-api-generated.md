## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|-------------|-------------|
| comment_id | String | Ja |  |
| spam | bool | Nein |  |
| perm_not_spam | bool | Nein |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für post_set_comment_spam_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_set_spam_status() -> Result<(), Error> {
    let params: PostSetCommentSpamStatusParams = PostSetCommentSpamStatusParams {
        comment_id: String::from("acme-news/2026/06/19/article-84/comment-1023"),
        spam: Some(true),
        perm_not_spam: Some(false),
        sso: Some(String::from("jwt:eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakepayload")),
    };
    let response: ApiEmptyResponse = post_set_comment_spam_status(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---