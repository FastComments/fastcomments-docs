## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| comment_id | String | はい |  |
| include_email | bool | いいえ |  |
| include_ip | bool | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

返却: [`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## 例

[inline-code-attrs-start title = 'get_moderation_comment の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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