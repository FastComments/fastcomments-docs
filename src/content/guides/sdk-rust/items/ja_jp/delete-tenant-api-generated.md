---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| sure | String | いいえ |  |

## レスポンス

戻り値: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'delete_tenant の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteTenantParams = DeleteTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "tenant-5f2d".to_string(),
        sure: Some("confirm".to_string()),
    };
    let response: ApiEmptyResponse = delete_tenant(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]

---