## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| comment_id | String | Ja |  |
| include_email | bool | Nein |  |
| include_ip | bool | Nein |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_moderation_comment Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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