## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| comment_id | String | 예 |  |
| approved | bool | 아니오 |  |
| broadcast_id | String | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_approved_response.rs)

## 예시

[inline-code-attrs-start title = 'post_set_comment_approval_status 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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