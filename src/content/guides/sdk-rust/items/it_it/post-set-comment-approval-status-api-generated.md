## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| approved | bool | No |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## Risposta

Restituisce: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_comment_approved_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio post_set_comment_approval_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn approve_comment(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PostSetCommentApprovalStatusParams {
        tenant_id: "acme-corp".to_string(),
        comment_id: "cmt-9876".to_string(),
        approved: Some(true),
        broadcast_id: Some("broadcast-2023".to_string()),
        sso: None,
    };
    let _response = post_set_comment_approval_status(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---