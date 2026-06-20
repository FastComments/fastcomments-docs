## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| comment_id | String | Ja |  |
| include_email | bool | Nee |  |
| include_ip | bool | Nee |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_moderation_comment Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comment() -> Result<ModerationApiCommentResponse, Error> {
    let params: GetModerationCommentParams = GetModerationCommentParams {
        comment_id: String::from("cmt-48291"),
        include_email: Some(true),
        include_ip: Some(false),
        sso: Some(String::from("sso-acme-corp-2026-token")),
    };
    let response: ModerationApiCommentResponse = get_moderation_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---