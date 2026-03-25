## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| delete_comments | String | Nein |  |
| comment_delete_mode | String | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'delete_tenant_user Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteTenantUserParams = DeleteTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9876".to_string(),
        delete_comments: Some("true".to_string()),
        comment_delete_mode: Some("permanent".to_string()),
    };
    let response: FlagCommentPublic200Response = delete_tenant_user(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---