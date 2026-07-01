## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| approved | bool | 否 |  |
| broadcast_id | String | 否 |  |
| sso | String | 否 |  |

## 回應

返回：[`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_approved_response.rs)

## 範例

[inline-code-attrs-start title = 'post_set_comment_approval_status 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn approve_comment(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PostSetCommentApprovalStatusParams {
        tenant_id: "acme-corp".to_string(),
        comment_id: "cmt-9876".to_string(),
        approved: Some(true),
        broadcast_id: Some("broadcast-2023".to_string()),
        sso: None,
    };
    let _response = post_set_comment_approval_status(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---