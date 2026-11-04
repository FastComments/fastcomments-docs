## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| user_id | String | 否 |  |
| anon_user_id | String | 否 |  |

## 回應

返回：[`FlagCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_response.rs)

## 範例

[inline-code-attrs-start title = 'flag_comment 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = FlagCommentParams {
        tenant_id: "acme-corp".to_string(),
        id: "comment-9876".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let _response = flag_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]