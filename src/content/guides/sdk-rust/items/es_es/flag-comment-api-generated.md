## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| id | String | Sí |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## Respuesta

Devuelve: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_200_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'flag_comment Ejemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<FlagComment200Response, Error> {
    let params: FlagCommentParams = FlagCommentParams {
        tenant_id: "acme-news-tenant".to_string(),
        id: "comment-20260325-842".to_string(),
        user_id: Some("user-7b2f3d".to_string()),
        anon_user_id: Some("anon-1a2b3c".to_string()),
    };
    let resp: FlagComment200Response = flag_comment(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]

---