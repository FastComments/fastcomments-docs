## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| comment_id | String | Sí |  |
| is_flagged | bool | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'flag_comment_public Ejemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_flag_comment() -> Result<(), Error> {
    let params: FlagCommentPublicParams = FlagCommentPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("comment-89b3"),
        is_flagged: true,
        sso: Some(String::from("sso-uid-7a2f")),
    };

    let _response: ApiEmptyResponse = flag_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---