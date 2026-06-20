## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| comment_id | String | Yes |  |
| set_comment_text_params | models::SetCommentTextParams | Yes |  |
| sso | String | No |  |

## Odgovor

Vraća: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_text_response.rs)

## Primjer

[inline-code-attrs-start title = 'post_set_comment_text Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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