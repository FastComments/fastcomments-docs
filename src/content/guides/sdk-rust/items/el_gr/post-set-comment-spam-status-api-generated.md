## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| comment_id | String | Yes |  |
| spam | bool | No |  |
| perm_not_spam | bool | No |  |
| sso | String | No |  |

## Απόκριση

Επιστρέφει: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα post_set_comment_spam_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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