## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| comment_id | String | 예 |  |
| include_email | bool | 아니요 |  |
| include_ip | bool | 아니요 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## 예제

[inline-code-attrs-start title = 'get_moderation_comment 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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