## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenant_id | String | Sí |  |
| skip | f64 | No |  |

## Respuesta

Devuelve: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_configs_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_question_configs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_question_configs() -> Result<GetQuestionConfigsResponse, Error> {
    let params: GetQuestionConfigsParams = GetQuestionConfigsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let response: GetQuestionConfigsResponse = get_question_configs(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---