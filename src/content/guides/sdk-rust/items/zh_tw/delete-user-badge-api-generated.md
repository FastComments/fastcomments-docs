## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 回應

回傳: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_badge_200_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_user_badge 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_badge() -> Result<UpdateUserBadge200Response, Error> {
    let params: DeleteUserBadgeParams = DeleteUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-7890".to_string(),
    };
    let response: UpdateUserBadge200Response = delete_user_badge(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---