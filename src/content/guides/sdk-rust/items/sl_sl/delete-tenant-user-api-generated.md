## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| delete_comments | String | Ne |  |
| comment_delete_mode | String | Ne |  |

## Odgovor

Vrača: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Primer

[inline-code-attrs-start title = 'delete_tenant_user Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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