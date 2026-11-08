## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |
| edit_key | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'delete_vote उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteVoteParams {
        tenant_id: "acme-corp".to_string(),
        id: "vote-12345".to_string(),
        edit_key: Some("edit-key-abc".to_string()),
    };
    let _response: VoteDeleteResponse = delete_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]