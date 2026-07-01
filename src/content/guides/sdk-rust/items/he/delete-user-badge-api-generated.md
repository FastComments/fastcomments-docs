## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## תגובה

מחזיר: [`ApiEmptySuccessResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_success_response.rs)

## דוגמה

[inline-code-attrs-start title = 'delete_user_badge דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn remove_badge(config: &configuration::Configuration) -> Result<(), Error> {
    let params: DeleteUserBadgeParams = DeleteUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-abcde".to_string(),
    };
    let _ = delete_user_badge(config, params).await?;
    Ok(())
}
[inline-code-end]