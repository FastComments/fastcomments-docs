## Parametri

| Naziv | Vrsta | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| broadcast_id | String | Da |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`PinComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pin_comment_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer un_pin_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_unpin() -> Result<(), Error> {
    let params: UnPinCommentParams = UnPinCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-20260325-7a9".to_string(),
        broadcast_id: "news/homepage/launch-article".to_string(),
        sso: Some("sso-jwt-user-0a1b2c3d".to_string()),
    };
    let response: PinComment200Response = un_pin_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---