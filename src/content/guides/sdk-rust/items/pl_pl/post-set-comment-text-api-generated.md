## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| comment_id | String | Tak |  |
| set_comment_text_params | models::SetCommentTextParams | Tak |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład post_set_comment_text'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_comment_text() -> Result<(), Error> {
    let params: PostSetCommentTextParams = PostSetCommentTextParams {
        comment_id: "comment-73b2a9".to_string(),
        set_comment_text_params: models::SetCommentTextParams {
            text: "Updated: The event now starts at 9:00 AM local time.".to_string(),
        },
        sso: Some("sso-session-8a7f3b".to_string()),
    };

    let response: SetCommentTextResponse = post_set_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---