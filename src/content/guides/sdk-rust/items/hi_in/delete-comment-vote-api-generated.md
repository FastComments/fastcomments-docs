## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| comment_id | String | हाँ |  |
| vote_id | String | हाँ |  |
| url_id | String | हाँ |  |
| broadcast_id | String | हाँ |  |
| edit_key | String | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'delete_comment_vote उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let params = DeleteCommentVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        vote_id: "vote-67890".to_string(),
        url_id: "news/article".to_string(),
        broadcast_id: "broadcast-abc".to_string(),
        edit_key: Some("edit-key-xyz".to_string()),
        sso: Some("sso-token-123".to_string()),
    };
    let _response: VoteDeleteResponse = delete_comment_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]