## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| id | String | Sì |  |
| replace_tenant_user_body | models::ReplaceTenantUserBody | Sì |  |
| update_comments | String | No |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di replace_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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