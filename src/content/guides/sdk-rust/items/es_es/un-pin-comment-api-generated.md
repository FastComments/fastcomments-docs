## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| comment_id | String | Sí |  |
| broadcast_id | String | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`PinComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pin_comment_200_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'un_pin_comment Ejemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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