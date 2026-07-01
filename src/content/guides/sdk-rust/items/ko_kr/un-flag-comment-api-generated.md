## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## 응답

반환: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_response.rs)

## 예시

[inline-code-attrs-start title = 'un_flag_comment 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = UnFlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-12345".to_string(),
        user_id: Some("user-67890".to_string()),
        anon_user_id: None,
    };
    let _response = un_flag_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]