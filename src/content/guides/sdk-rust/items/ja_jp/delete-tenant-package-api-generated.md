---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |

## レスポンス

戻り値: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'delete_tenant_package の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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