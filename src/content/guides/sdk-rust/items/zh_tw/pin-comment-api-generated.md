## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| broadcast_id | String | 是 |  |
| sso | String | 否 |  |

## 回應

回傳：[`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_comment_pin_status_response.rs)

## 範例

[inline-code-attrs-start title = 'pin_comment 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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