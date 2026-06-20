## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| include_email | bool | 否 |  |
| include_ip | bool | 否 |  |
| sso | String | 否 |  |

## 回應

回傳： [`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## 範例

[inline-code-attrs-start title = 'get_moderation_comment 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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