## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| replace_tenant_user_body | models::ReplaceTenantUserBody | 是 |  |
| update_comments | String | 否 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 示例

[inline-code-attrs-start title = 'replace_tenant_user 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: ReplaceTenantUserParams = ReplaceTenantUserParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "user-42".to_string(),
    replace_tenant_user_body: models::ReplaceTenantUserBody {
        user_id: "user-42".to_string(),
        display_name: "Jane Doe".to_string(),
        email: "jane.doe@acme.com".to_string(),
        roles: vec!["moderator".to_string()],
    },
    update_comments: Some("true".to_string()),
};
let response: FlagCommentPublic200Response = replace_tenant_user(&configuration, params).await?
[inline-code-end]

---