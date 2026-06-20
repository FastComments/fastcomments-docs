## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| url_id | String | Sí |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de put_reopen_thread'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_reopen_thread() -> Result<(), Error> {
    let params: PutReopenThreadParams = PutReopenThreadParams {
        url_id: String::from("acme-corp/news/article-2026-06-19"),
        sso: Some(String::from("sso-token-9f8e7d6c")),
    };
    let response: ApiEmptyResponse = put_reopen_thread(configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---