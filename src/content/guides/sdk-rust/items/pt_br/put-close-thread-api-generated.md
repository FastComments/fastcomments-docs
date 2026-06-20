## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| url_id | String | Sim |  |
| sso | String | Não |  |

## Resposta

Retorna: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Exemplo

[inline-code-attrs-start title = 'put_close_thread Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn close_thread() -> Result<(), Error> {
    let params: PutCloseThreadParams = PutCloseThreadParams {
        url_id: String::from("news/2026/07/acme-launch-coverage"),
        sso: Some(String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.signature")),
    };
    let response: ApiEmptyResponse = put_close_thread(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---