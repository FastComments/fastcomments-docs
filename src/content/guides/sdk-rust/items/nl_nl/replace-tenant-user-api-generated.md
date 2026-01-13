## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| replace_tenant_user_body | models::ReplaceTenantUserBody | Ja |  |
| update_comments | String | Nee |  |

## Respons

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'replace_tenant_user Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let cfg: &configuration::Configuration = &configuration;
let replace_tenant_user_body: models::ReplaceTenantUserBody = models::ReplaceTenantUserBody {
    external_id: Some("acct-834".to_string()),
    email: Some("jane.doe@acme-news.com".to_string()),
    display_name: Some("Jane Doe".to_string()),
    role: Some("moderator".to_string()),
};
let params: ReplaceTenantUserParams = ReplaceTenantUserParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "user-834".to_string(),
    replace_tenant_user_body,
    update_comments: Some("true".to_string()),
};
let resp: FlagCommentPublic200Response = replace_tenant_user(cfg, params).await?;
[inline-code-end]

---