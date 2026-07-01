## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| delete_comments | String | No |  |
| comment_delete_mode | String | No |  |

## 回應

回傳：[`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_tenant_user 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteTenantUserParams {
        tenant_id: "acme-corp".into(),
        id: "user-123".into(),
        delete_comments: Some("true".into()),
        comment_delete_mode: Some("hard".into()),
    };
    delete_tenant_user(&config, params).await?;
    Ok(())
}
[inline-code-end]

---