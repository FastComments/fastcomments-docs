## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| replace_tenant_user_body | models::ReplaceTenantUserBody | 是 |  |
| update_comments | String | 否 |  |

## 响应

返回：[`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'replace_tenant_user 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = ReplaceTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-12345".to_string(),
        replace_tenant_user_body: ReplaceTenantUserBody::default(),
        update_comments: Some("Update user role".to_string()),
    };
    replace_tenant_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---