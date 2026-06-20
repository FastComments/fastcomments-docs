## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|-------------|
| tenant_id | String | 예 |  |
| comment_id | String | 예 |  |
| broadcast_id | String | 예 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_comment_pin_status_response.rs)

## 예제

[inline-code-attrs-start title = 'pin_comment 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_pin() -> Result<ChangeCommentPinStatusResponse, Error> {
    let params: PinCommentParams = PinCommentParams {
        tenant_id: "acme-news".to_string(),
        comment_id: "cmt-9f8b7d6".to_string(),
        broadcast_id: "news/article/2026/06/19/article-12345".to_string(),
        sso: Some("user-ssotoken-abc123".to_string()),
    };
    let response: ChangeCommentPinStatusResponse = pin_comment(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---