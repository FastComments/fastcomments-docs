## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| create_tenant_user_body | models::CreateTenantUserBody | Evet |  |

## Yanıt

Döndürür: [`CreateTenantUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_user_response.rs)

## Örnek

[inline-code-attrs-start title = 'create_tenant_user Örnek'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = CreateTenantUserParams {
        tenant_id: "acme-corp".to_string(),
        create_tenant_user_body: models::CreateTenantUserBody {
            email: "john.doe@example.com".to_string(),
            role: "admin".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            digest_email_frequency: Some(DigestEmailFrequency::Daily),
            imported_agent_approval_notification_frequency: Some(ImportedAgentApprovalNotificationFrequency::Weekly),
            ..Default::default()
        },
    };
    let _response: CreateTenantUserResponse = create_tenant_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]