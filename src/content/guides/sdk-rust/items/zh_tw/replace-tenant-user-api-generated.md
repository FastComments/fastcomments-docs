## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| replace_tenant_user_body | models::ReplaceTenantUserBody | 是 |  |
| update_comments | String | 否 |  |

## 回應

回傳：[`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 範例

[inline-code-attrs-start title = 'replace_tenant_user 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: ReplaceTenantUserParams = ReplaceTenantUserParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "user-123".to_string(),
    replace_tenant_user_body: models::ReplaceTenantUserBody {
        user_id: "user-123".to_string(),
        email: "jane.doe@acme.com".to_string(),
        display_name: "Jane Doe".to_string(),
        roles: vec!["editor".to_string()],
    },
    update_comments: Some("propagate".to_string()),
};

let response: ApiEmptyResponse = replace_tenant_user(&configuration, params).await?;
[inline-code-end]

---