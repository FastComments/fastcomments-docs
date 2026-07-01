## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| comment_id | String | Sì |  |
| broadcast_id | String | Sì |  |
| sso | String | No |  |

## Risposta

Ritorna: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Esempio

[inline-code-attrs-start title = 'un_lock_comment Esempio'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = UnLockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-456".to_string(),
        broadcast_id: "news/article-123".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _response = un_lock_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---