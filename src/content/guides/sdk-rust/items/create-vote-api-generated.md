## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| direction | String | Yes |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## Response

Returns: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_vote Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_create_vote() -> Result<VoteComment200Response, Error> {
    let params: CreateVoteParams = CreateVoteParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("news/article-12345/comment-6789"),
        direction: String::from("up"),
        user_id: Some(String::from("user-42")),
        anon_user_id: None,
    };
    let response: VoteComment200Response = create_vote(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
