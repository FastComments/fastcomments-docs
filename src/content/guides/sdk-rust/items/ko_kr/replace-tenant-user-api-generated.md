---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| replace_tenant_user_body | models::ReplaceTenantUserBody | 예 |  |
| update_comments | String | 아니요 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'replace_tenant_user 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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