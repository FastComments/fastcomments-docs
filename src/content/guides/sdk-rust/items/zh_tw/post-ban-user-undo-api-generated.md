## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| ban_user_undo_params | models::BanUserUndoParams | 是 |  |
| sso | String | 否 |  |

## 回應

返回：[`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 範例

[inline-code-attrs-start title = 'post_ban_user_undo 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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