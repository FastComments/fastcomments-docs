## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| delete_comments | String | 否 |  |
| comment_delete_mode | String | 否 |  |

## 响应

返回： [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'delete_tenant_user 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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