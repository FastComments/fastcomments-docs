## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| create_tenant_user_body | models::CreateTenantUserBody | Sì |  |

## Risposta

Restituisce: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_user_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio create_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CreateTenantUserParams = CreateTenantUserParams {
    tenant_id: "acme-corp-tenant".to_string(),
    create_tenant_user_body: models::CreateTenantUserBody {
        email: "jane.doe@acme.com".to_string(),
        display_name: Some("Jane Doe".to_string()),
        role: Some("moderator".to_string()),
        locale: Some("en-US".to_string()),
        digest_email_frequency: Some(DigestEmailFrequency::Daily),
        imported_agent_approval_notification_frequency: Some(ImportedAgentApprovalNotificationFrequency::Immediate),
    },
};
let created: CreateTenantUserResponse = create_tenant_user(&configuration, params).await?;
[inline-code-end]

---