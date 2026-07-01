## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| broadcast_id | String | Da |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_comment_pin_status_response.rs)

## Primjer

[inline-code-attrs-start title = 'pin_comment Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PinCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        broadcast_id: "news/article".to_string(),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _response = pin_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]