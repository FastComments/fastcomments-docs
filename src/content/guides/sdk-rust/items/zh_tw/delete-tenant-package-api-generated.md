---
## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 回應

回傳: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_tenant_package 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_example() -> Result<(), Error> {
    let params: DeleteTenantPackageParams = DeleteTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "premium-comment-moderation".to_string(),
    };
    let response: ApiEmptyResponse = delete_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---