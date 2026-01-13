## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| id | String | Tak |  |
| delete_comments | String | Nie |  |
| comment_delete_mode | String | Nie |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład delete_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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