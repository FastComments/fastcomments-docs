## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| delete_comments | String | いいえ |  |
| comment_delete_mode | String | いいえ |  |

## レスポンス

返却値: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'delete_tenant_user の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteTenantUserParams = DeleteTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-8421".to_string(),
        delete_comments: Some("yes".to_string()),
        comment_delete_mode: Some("permanent".to_string()),
    };
    let _response: ApiEmptyResponse = delete_tenant_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---