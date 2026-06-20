---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|-------------|
| ban_user_undo_params | models::BanUserUndoParams | 예 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'post_ban_user_undo 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn undo_ban_example() -> Result<ApiEmptyResponse, Error> {
    let ban_params: models::BanUserUndoParams = models::BanUserUndoParams {
        tenant_slug: "acme-corp-tenant".to_string(),
        user_id: "user-0042".to_string(),
        ban_id: "ban-2025-08-15-0001".to_string(),
        undone_by: "mod_stephanie".to_string(),
        note: Some("Ban reversed after review".to_string()),
    };
    let params: PostBanUserUndoParams = PostBanUserUndoParams {
        ban_user_undo_params: ban_params,
        sso: Some("https://sso.acme-corp.com/saml/response".to_string()),
    };
    let resp: ApiEmptyResponse = post_ban_user_undo(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]

---