## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| direction | String | Yes |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## 응답

반환: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## 예시

[inline-code-attrs-start title = 'create_vote 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn submit_vote() -> Result<(), Error> {
    let params = CreateVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345".to_string(),
        direction: "up".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: Some("anon-99".to_string()),
    };
    let _response = create_vote(&config, params).await?;
    Ok(())
}
[inline-code-end]