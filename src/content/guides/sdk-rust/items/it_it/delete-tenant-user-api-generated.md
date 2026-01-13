---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| id | String | Sì |  |
| delete_comments | String | No |  |
| comment_delete_mode | String | No |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di delete_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteTenantUserParams = DeleteTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-12345".to_string(),
        delete_comments: Some("true".to_string()),
        comment_delete_mode: Some("cascade".to_string()),
    };
    let resp: FlagCommentPublic200Response = delete_tenant_user(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]

---