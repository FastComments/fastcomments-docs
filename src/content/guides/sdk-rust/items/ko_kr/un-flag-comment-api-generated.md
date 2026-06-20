## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| user_id | String | 아니오 |  |
| anon_user_id | String | 아니오 |  |

## 응답

반환: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_response.rs)

## 예제

[inline-code-attrs-start title = 'un_flag_comment 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn unflag_example() -> Result<FlagCommentResponse, Error> {
    let params: UnFlagCommentParams = UnFlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-98765".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let response: FlagCommentResponse = un_flag_comment(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---