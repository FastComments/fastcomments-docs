## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenant_id | String | Sí |  |
| post_id | String | Sí |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_feed_post_public_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo delete_feed_post_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-123".to_string(),
        broadcast_id: Some("broadcast-456".to_string()),
        sso: Some("sso-token-789".to_string()),
    };
    let _response: DeleteFeedPostPublicResponse = delete_feed_post_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]