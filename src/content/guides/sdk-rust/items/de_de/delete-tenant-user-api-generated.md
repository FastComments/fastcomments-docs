---
## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| delete_comments | String | Nein |  |
| comment_delete_mode | String | Nein |  |

## Antwort

Gibt zurück: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Beispiel

[inline-code-attrs-start title = 'delete_tenant_user Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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