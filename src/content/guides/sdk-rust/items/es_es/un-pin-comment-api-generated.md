## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| comment_id | String | Sí |  |
| broadcast_id | String | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_comment_pin_status_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de un_pin_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<ChangeCommentPinStatusResponse, Error> {
    let params: UnPinCommentParams = UnPinCommentParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("cmt-8f3b2a1e"),
        broadcast_id: String::from("news/2024/product-launch"),
        sso: Some(String::from("sso-user-abcdef123456")),
    };
    let response: ChangeCommentPinStatusResponse = un_pin_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---