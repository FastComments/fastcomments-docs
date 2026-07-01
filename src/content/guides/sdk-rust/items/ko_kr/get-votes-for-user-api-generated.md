## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## 응답

반환: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_for_user_response.rs)

## 예시

[inline-code-attrs-start title = 'get_votes_for_user 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetVotesForUserParams {
        tenant_id: "acme-corp".to_string(),
        url_id: "news/2023/09/awesome-article".to_string(),
        user_id: Some("user-12345".to_string()),
        anon_user_id: None,
    };
    let _response = get_votes_for_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]