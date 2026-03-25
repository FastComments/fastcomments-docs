## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| direction | String | Yes |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## 返回

返回: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## 示例

[inline-code-attrs-start title = 'create_vote 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_create_vote() -> Result<(), Error> {
    let params: CreateVoteParams = CreateVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345".to_string(),
        direction: "up".to_string(),
        user_id: Some("user-9876".to_string()),
        anon_user_id: Some("anon-01-abcdef".to_string()),
    };

    let response: VoteComment200Response = create_vote(&configuration, params).await?;
    println!("{:?}", response);
    Ok(())
}
[inline-code-end]

---