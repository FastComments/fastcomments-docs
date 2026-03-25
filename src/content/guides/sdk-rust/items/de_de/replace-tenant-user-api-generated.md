## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| replace_tenant_user_body | models::ReplaceTenantUserBody | Ja |  |
| update_comments | String | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'replace_tenant_user Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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