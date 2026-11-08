## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| approved | bool | नहीं |  |
| broadcast_id | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_approved_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'post_set_comment_approval_status उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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