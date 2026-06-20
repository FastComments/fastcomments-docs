## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| comment_id | String | Ja |  |
| sso | String | Nej |  |

## Respons

Returnerer: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_text_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_moderation_comment_text Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comment_text() -> Result<(), Error> {
    let params: GetModerationCommentTextParams = GetModerationCommentTextParams {
        comment_id: String::from("news/technology/2026/06/19/ai-ethics-12345"),
        sso: Some(String::from("sso-token-7f3a9b2c")),
    };
    let _response: GetCommentTextResponse = get_moderation_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---