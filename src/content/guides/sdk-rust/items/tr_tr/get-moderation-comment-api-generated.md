## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| comment_id | String | Evet |  |
| include_email | bool | Hayır |  |
| include_ip | bool | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Dönüş değeri: [`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_moderation_comment Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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