## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| ban_user_undo_params | models::BanUserUndoParams | Yes |  |
| sso | String | No |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예시

[inline-code-attrs-start title = 'post_ban_user_undo 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PostBanUserUndoParams {
        tenant_id: "acme-corp-tenant".to_string(),
        ban_user_undo_params: models::BanUserUndoParams {
            user_id: "user-42".to_string(),
            note: Some("ban appeal accepted".to_string()),
        },
        sso: Some("sso-token-abc".to_string()),
    };
    let _ = post_ban_user_undo(&configuration, params).await?;
    Ok(())
}
[inline-code-end]